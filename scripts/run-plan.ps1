# Runs docs/PLAN.json unattended: one task per omp session, in its own checkout, merged to master only if it
# proves itself. Stops on its own.
#
#   scripts/run-plan.ps1                 # up to 3 tasks, 25 minutes each
#   scripts/run-plan.ps1 -Max 8 -Minutes 40
#   scripts/run-plan.ps1 -DryRun         # print what it would do, run nothing
#
# The shape is Kevin's rule from AGENTS.md: a long-running worker gets its own branch, the integration owner
# merges. Here the worker is an omp session and the integration owner is this script - and the worker gets its own
# CHECKOUT as well:
#
#   C:\Users\kevin\iimage            the MAIN tree. Always on master, never checked out to a task branch and never
#                                    edited by a worker. Merges, pushes and the post-merge test happen here.
#   C:\Users\kevin\iimage-work\<id>  one git worktree per task, created with
#                                    `git worktree add -B plan/<id> <tree> master`; omp is started with --cwd
#                                    there, cargo test runs there, and the tree is removed after the merge.
#
# Why the checkout is separate: the old lifecycle checked the task branch out in the main tree, so worker and
# driver shared one checkout. A worker that left files behind made the next `git checkout master` fail; a stray
# `git worktree add ... master` (from a baseline comparison) made it impossible; and a failed task was
# recoverable only from a stash. Two checkouts remove all three - the driver never has to change the main tree's
# branch, a leftover tree cannot block it, and a failed task's checkout is simply kept and named.
#
# Per task:
#   1. master must be clean and up to date, or nothing starts
#   1b. a tree left by an earlier run is decided about, never assumed: a clean tree whose branch holds nothing but
#       the ledger note is removed and the task starts fresh, real work in it makes the task REFUSED (path and
#       branch printed) and the loop goes on to the next pending task - a kept tree must not cost a whole round
#   2. the task tree is created from master with branch plan/<id>
#   3. the session works there; it may commit, it may NOT merge or push master
#   4. the ledger in that tree must record a result, or the task is blocked (silence is failure)
#   5. the branch must contain a commit, or the task is blocked
#   6. cargo test must pass in the task tree, and the tree must be clean, or the task is blocked
#   7. merge --no-ff into master from the MAIN tree, test again there, push, remove the tree, delete the branch
#   8. a post-merge test failure resets master to where it was and blocks the task
#   9. on failure the tree is KEPT, the blocked ledger is committed on its branch, and its path is printed - the
#      failure is inspectable instead of reconstructed from a stash
#
# One worker at a time. The trees are per task and the work root is a parameter, so parallel workers are a later
# change to the loop rather than a rewrite of it.
#
# Guards: docs/PLAN.STOP halts before starting; a blocked task stops the whole loop; unattended is the wrong place
# to try a second approach. Every run is logged to logs/plan-<stamp>-<id>.log.

param(
    [int]$Max = 3,
    [int]$Minutes = 25,
    [string]$Repo = "C:\Users\kevin\rust_lessons",
    [string]$Omp = "C:\Users\kevin\.local\bin\omp.exe",
    [string]$Remote = "origin",
    [string]$Main = "main",
    [string]$WorkRoot = "",
    [int]$MaxPending = 16,
    [string]$Worker = 'w1',
    [switch]$SkipBlocked,
    [switch]$UntilDone,
    [switch]$DryRun
)

$ErrorActionPreference = 'Stop'
$Repo = (Resolve-Path -LiteralPath $Repo).Path.TrimEnd('\')
# The work root is a sibling of the repository: C:\Users\kevin\iimage -> C:\Users\kevin\iimage-work
if (-not $WorkRoot) { $WorkRoot = Join-Path (Split-Path -Parent $Repo) ((Split-Path -Leaf $Repo) + '-work') }
$WorkRoot = $WorkRoot.TrimEnd('\')
if ($WorkRoot.StartsWith($Repo + '\') -or $WorkRoot -eq $Repo) {
    throw "the work root must live outside the repository: $WorkRoot is inside $Repo"
}
$stopFile = Join-Path $Repo 'docs\PLAN.STOP'
$logDir = Join-Path $Repo 'logs'
New-Item -ItemType Directory -Force -Path $logDir | Out-Null
$keptTrees = @()
$blockedNow = @()
$refused = @()


# git writes its normal chatter ("Switched to a new branch") to stderr, and $ErrorActionPreference = 'Stop'
# turns that into a fatal NativeCommandError. Success is the exit code, never the stream.
function Invoke-GitIn([string]$Where, [string[]]$Arguments) {
    $previous = $ErrorActionPreference
    $ErrorActionPreference = 'Continue'
    try {
        $output = & git -C $Where @Arguments 2>&1
        $code = $LASTEXITCODE
    } finally {
        $ErrorActionPreference = $previous
    }
    if ($code -ne 0) { throw ("git {0} failed in {1}: {2}" -f ($Arguments -join ' '), $Where, ($output -join "`n")) }
    return $output
}

function Invoke-Git([string[]]$Arguments) { return Invoke-GitIn $Repo $Arguments }

function Invoke-GitQuietIn([string]$Where, [string[]]$Arguments) {
    $previous = $ErrorActionPreference
    $ErrorActionPreference = 'Continue'
    try { return (& git -C $Where @Arguments 2>&1) } finally { $ErrorActionPreference = $previous }
}

function Invoke-GitQuiet([string[]]$Arguments) { return Invoke-GitQuietIn $Repo $Arguments }

# A yes/no git question: the exit code is the answer, and the output (which git writes to both streams) is not.
function Test-Git([string[]]$Arguments) {
    $previous = $ErrorActionPreference
    $ErrorActionPreference = 'Continue'
    try {
        $null = & git -C $Repo @Arguments 2>&1
        return ($LASTEXITCODE -eq 0)
    } finally {
        $ErrorActionPreference = $previous
    }
}

function Get-LedgerPath([string]$Root) {
    # The ledger lives on the task's branch, so the task's tree is the normal place. But that tree is disposable - the driver removes
    # it on success and keeps it on failure - and a missing tree must not kill the run: on 2026-11-07 a removed worktree made
    # Set-TaskStatus throw 'Cannot find path ...\docs\PLAN.json' and the whole round exited 1 AFTER the work had merged. Fall back to
    # the repository's own ledger, and say so, because a silent fallback would hide a lifecycle bug rather than survive one.
    $task = Join-Path $Root 'docs\PLAN.json'
    if (Test-Path -LiteralPath $task) { return $task }
    $repo = Join-Path $Repo 'docs\PLAN.json'
    if (Test-Path -LiteralPath $repo) {
        Write-Output ("ledger: task tree {0} is gone, recording status in the repository ledger" -f $Root)
        return $repo
    }
    return $task
}

# The ledger lives on the task's branch, so it is read and written in that task's tree - never in the main tree,
# which stays on master and clean.
function Read-Ledger([string]$Root) { Get-Content -Raw -LiteralPath (Get-LedgerPath $Root) | ConvertFrom-Json }

# The right budget is the one the ledger measured: p90 of the finished tasks, with headroom, clamped. 10 minutes was
# too tight for a 424-line deletion and 20 was too tight as well; guessing a constant fails in both directions.
function Get-Budget($ledger, [int]$fallback) {
    $spent = @($ledger.tasks | Where-Object { $_.PSObject.Properties.Name -contains 'minutes' -and $_.minutes } |
        ForEach-Object { [double]$_.minutes } | Sort-Object)
    if ($spent.Count -lt 3) { return $fallback }
    $index = [math]::Min($spent.Count - 1, [int][math]::Ceiling($spent.Count * 0.9) - 1)
    $p90 = $spent[$index]
    return [int]([math]::Min(45, [math]::Max(8, $p90 * 1.5)))
}

# Two drivers in one repository need three things the single-worker loop never did: a task can be CLAIMED so two workers never take
# the same one, the ledger write is atomic under a lock, and merges are serialised because two merges racing on master is a lost commit.
# Directory creation is atomic on Windows, so it is the lock; a lock older than two minutes is stale and is cleared, because no claim
# or merge takes that long and a crashed worker must not wedge the repository.
function Enter-PlanLock([string]$Name) {
    $path = Join-Path $Repo ".git\plan-$Name.lock"
    for ($i = 0; $i -lt 120; $i++) {
        try {
            New-Item -ItemType Directory -Path $path -ErrorAction Stop | Out-Null
            return $path
        } catch {
            if (Test-Path -LiteralPath $path) {
                $age = (Get-Date) - (Get-Item -LiteralPath $path).CreationTime
                if ($age.TotalMinutes -gt 2) { Remove-Item -LiteralPath $path -Recurse -Force -ErrorAction SilentlyContinue }
            }
            Start-Sleep -Milliseconds 250
        }
    }
    throw ("could not take the {0} lock after 30 s" -f $Name)
}

function Exit-PlanLock([string]$Path) {
    if ($Path) { Remove-Item -LiteralPath $Path -Recurse -Force -ErrorAction SilentlyContinue }
}

# A worker that exits - finished, crashed or refused - must hand back anything it claimed, or the task stays 'running' forever and
# nothing else can pick it up. That is exactly what happened to two claims on 2026-11-07: the workers exited and left gate-the-python-checks
# and prove-omp-cwd claimed by dead pids.
function Release-Claims([string]$Worker) {
    $lock = Enter-PlanLock 'ledger'
    try {
        $ledger = Read-Ledger $Repo
        $released = @()
        foreach ($t in $ledger.tasks) {
            if ($t.status -eq 'running' -and $t.PSObject.Properties['owner'] -and $t.owner -eq $Worker) {
                $t.status = 'pending'
                $t.PSObject.Properties.Remove('owner')
                $t.PSObject.Properties.Remove('claimed')
                $released += $t.id
            }
        }
        if ($released.Count -gt 0) {
            [System.IO.File]::WriteAllText((Join-Path $Repo 'docs\PLAN.json'), ($ledger | ConvertTo-Json -Depth 8), (New-Object System.Text.UTF8Encoding($false)))
            Write-Output ("released {0} unclaimed by {1}" -f ($released -join ', '), $Worker)
        }
    } finally {
        Exit-PlanLock $lock
    }
}

# Claim: pick a pending task and mark it running for this worker, in one locked read-modify-write, so the second worker sees it gone.
function Claim-Task([string]$Worker, [string[]]$Skip) {
    $lock = Enter-PlanLock 'ledger'
    try {
        $ledger = Read-Ledger $Repo
        $task = $ledger.tasks | Where-Object { $_.status -eq 'pending' -and $Skip -notcontains $_.id } | Select-Object -First 1
        if (-not $task) { return $null }
        foreach ($t in $ledger.tasks) {
            if ($t.id -eq $task.id) {
                $t.status = 'running'
                $t | Add-Member -NotePropertyName owner -NotePropertyValue $Worker -Force
                $t | Add-Member -NotePropertyName claimed -NotePropertyValue (Get-Date -Format 'yyyy-MM-dd HH:mm') -Force
            }
        }
        [System.IO.File]::WriteAllText((Join-Path $Repo 'docs\PLAN.json'), ($ledger | ConvertTo-Json -Depth 8), (New-Object System.Text.UTF8Encoding($false)))
        Write-Output ("claimed {0} for worker {1}" -f $task.id, $Worker)
        return $task
    } finally {
        Exit-PlanLock $lock
    }
}

function Get-NextTask($ledger, $skip = @()) {
    $ledger.tasks | Where-Object { $_.status -eq 'pending' -and $skip -notcontains $_.id } | Select-Object -First 1
}

function Set-TaskStatus([string]$Root, [string]$id, [string]$status, [string]$note) {
    $ledger = Read-Ledger $Root
    foreach ($task in $ledger.tasks) {
        if ($task.id -eq $id) {
            $task.status = $status
            if ($note) { $task | Add-Member -NotePropertyName note -NotePropertyValue $note -Force }
            if ($status -ne 'pending') {
                $task | Add-Member -NotePropertyName finished -NotePropertyValue (Get-Date -Format 'yyyy-MM-dd HH:mm:ss') -Force
            }
        }
    }
    $json = $ledger | ConvertTo-Json -Depth 8
    # UTF8 without BOM: the app's JSON reader (and python's) reject a BOM, and Set-Content -Encoding UTF8 adds one
    [System.IO.File]::WriteAllText((Get-LedgerPath $Root), $json, (New-Object System.Text.UTF8Encoding($false)))
}

function Add-Minutes([string]$Root, [string]$id, [int]$minutes) {
    if ($minutes -le 0) { return }
    $ledger = Read-Ledger $Root
    foreach ($task in $ledger.tasks) {
        if ($task.id -eq $id) { $task | Add-Member -NotePropertyName minutes -NotePropertyValue $minutes -Force }
    }
    [System.IO.File]::WriteAllText((Get-LedgerPath $Root), ($ledger | ConvertTo-Json -Depth 8), (New-Object System.Text.UTF8Encoding($false)))
}

# A result the branch will not carry is a result a person cannot read after the run: commit it in the task's tree.
function Commit-Ledger([string]$Root, [string]$message) {
    Invoke-GitIn $Root @('add', '--', 'docs/PLAN.json') | Out-Null
    Invoke-GitIn $Root @('commit', '-q', '-m', $message) | Out-Null
}

function Get-TaskTree([string]$id) { Join-Path $WorkRoot $id }

# Which checkout in this repository holds which branch. `git worktree list --porcelain` prints one block per
# worktree, `worktree <path>` first and `branch refs/heads/<name>` (or `detached`) after it.
function Get-WorktreeBranches {
    $map = @{}
    $path = $null
    foreach ($line in @(Invoke-GitQuiet @('worktree', 'list', '--porcelain'))) {
        if ($line -like 'worktree *') {
            $path = (($line.Substring(9).Trim()) -replace '/', '\')
            $map[$path] = ''
        } elseif ($path -and $line -like 'branch refs/heads/*') {
            $map[$path] = $line.Trim().Substring(18)
        } elseif ($path -and $line.Trim() -eq 'detached') {
            $map[$path] = 'detached'
        }
    }
    return $map
}

# The files a branch changes that are not the ledger: the ledger note is a status record, not work, and a task
# whose branch carries nothing else has nothing worth refusing over.
function Get-FilesOutsideLedger([string]$branch) {
    return @(Invoke-GitQuiet @('diff', '--name-only', "$Main..$branch") |
        Where-Object { $_ -and $_.Trim() -and $_.Trim() -ne 'docs/PLAN.json' } | ForEach-Object { $_.Trim() })
}

# What to do about a path the task wants to use, decided explicitly and reported as data:
#   @{ ok = $true;  action = 'new' }                     nothing in the way
#   @{ ok = $true;  action = 'reclaim'; note = '<text>' }  a clean tree holding only the ledger note: remove it
#   @{ ok = $false; reason = '<text>' }                    real work, or a tree that is not ours to touch: refuse
#
# Measured failure this exists for (2026-11-06): a blocked task kept its tree as evidence, and the next run started
# 0 tasks because `git worktree add -B plan/<id> <tree>` cannot reuse an existing path. Refusing the TASK and
# carrying on is what keeps one kept tree from costing a whole round; the path, the branch and the way to clear it
# are in the reason, so the evidence is still inspectable.
function Get-KeptTreeVerdict([string]$id, [string]$tree) {
    $branch = "plan/$id"
    $key = $tree -replace '/', '\'
    $holds = Get-WorktreeBranches
    $clear = "inspect it with: git -C {0} status; clear it with: git -C {1} worktree remove --force {0} and git -C {1} branch -D {2}" -f $tree, $Repo, $branch

    if ($holds.ContainsKey($key)) {
        if ($holds[$key] -ne $branch) {
            return @{ ok = $false; reason = ("{0} is a worktree of this repository already holding '{1}', not {2}; {3}" -f $tree, $holds[$key], $branch, $clear) }
        }
        $dirty = ((Invoke-GitQuietIn $tree @('status', '--porcelain')) -join "`n").Trim()
        if ($dirty) {
            $shown = @($dirty -split "`n" | Select-Object -First 5) -join '; '
            return @{ ok = $false; reason = ("the kept tree at {0} (branch {1}) holds uncommitted work: {2}; {3}" -f $tree, $branch, $shown, $clear) }
        }
        $outside = @()
        if (Test-Git @('rev-parse', '--verify', '--quiet', "refs/heads/$branch")) {
            $outside = Get-FilesOutsideLedger $branch
        }
        if ($outside.Count) {
            $count = ((Invoke-GitQuiet @('rev-list', '--count', "$Main..$branch")) -join '').Trim()
            return @{ ok = $false; reason = ("the kept tree at {0} (branch {1}) holds {2} commit(s) changing {3}; {4}" -f $tree, $branch, $count, (($outside | Select-Object -First 5) -join ', '), $clear) }
        }
        return @{ ok = $true; action = 'reclaim'; tree = $tree; branch = $branch }
    }

    if (Test-Path -LiteralPath $tree) {
        # a directory git does not list as a worktree: deleting it would delete something this driver did not make
        return @{ ok = $false; reason = ("{0} exists but is not a worktree of {1}, so it is not this driver's to remove; move it aside or add it back with git -C {1} worktree add {0} {2}" -f $tree, $Repo, $branch) }
    }
    foreach ($other in $holds.Keys) {
        if ($holds[$other] -eq $branch) {
            return @{ ok = $false; reason = ("branch {0} is checked out at {1}, not at {2}; that is a kept tree from another run: inspect it with git -C {1} status, then: git -C {3} worktree remove --force {1}" -f $branch, $other, $tree, $Repo) }
        }
    }
    if (Test-Git @('rev-parse', '--verify', '--quiet', "refs/heads/$branch")) {
        $outside = Get-FilesOutsideLedger $branch
        if ($outside.Count) {
            $count = ((Invoke-GitQuiet @('rev-list', '--count', "$Main..$branch")) -join '').Trim()
            return @{ ok = $false; reason = ("branch {0} exists with {1} commit(s) that are not in {2}, changing {3}; inspect with git -C {4} log {2}..{0}, and merge or delete it before running again" -f $branch, $count, $Main, (($outside | Select-Object -First 5) -join ', '), $Repo) }
        }
        # the branch is left over but carries only a ledger note: `worktree add -B` may reset it, and that loses a
        # status line nobody needs once the task is running again
    }
    return @{ ok = $true; action = 'new' }
}

function New-TaskTree([string]$id, [string]$tree) {
    New-Item -ItemType Directory -Force -Path $WorkRoot | Out-Null
    Invoke-Git @('worktree', 'add', '-B', "plan/$id", $tree, $Main) | Out-Null
}

# No --force: a clean tree is removed, and `worktree remove` itself is the assertion that the session committed
# everything. A dirty tree is never deleted - it is a failure that keeps its evidence.
function Remove-TaskTree([string]$tree) {
    Invoke-Git @('worktree', 'remove', $tree) | Out-Null
    Invoke-Git @('worktree', 'prune') | Out-Null
}

function Invoke-Task($task, [string]$tree, [string]$branch, [string]$logPath) {
    $brief = @"
You are working in $tree on ONE task from docs/PLAN.json.

$tree is your own git worktree of $Repo, on branch '$branch'. Everything you write goes there. $Repo is the
main tree: it stays on master and the driver merges from it, so do not edit it, do not check out anything there,
and do not remove a worktree.

TASK ID: $($task.id)
TITLE: $($task.title)
ACCEPTANCE: $($task.acceptance)

You are already on branch '$branch'. Work only here.

Rules that are not negotiable:
- Do only this task. Do not start the next one.
- Prove it: a test, a check, or a measurement written into docs/RESUME.md. Claims without evidence are failures.
- Commit on this branch as you go, following the version rule in AGENTS.md (bug -> patch, feature -> minor).
- Do NOT merge, rebase or push master. The driver merges and pushes; you push nothing.
- Leave the tree clean when you finish: the driver removes it, and it will not remove one with uncommitted work.
- Write this turn's notes in the note file the driver created for you under notes\ - one file per turn, named
  <date>-<task-id>-<worker>.txt. README.md is reserved for long-lived product information: do NOT append a changelog entry to it,
  because every turn doing that is exactly what makes two rounds conflict on prose. Never edit notes\README.md, notes\INDEX.md or
  another turn's note.
- You MAY append a task you discovered, as status "pending", origin "worker", with an acceptance line - but you
  may never mark another task done, reopen a blocked one, or edit an acceptance.
- No windows: this project has no desktop harness. Verify with `cargo test` plus a recorded command, never by opening the app.
- Never touch C:/Users/kevin/OneDrive/Pictures (the real gallery) and never publish: no `gh release`, no push to
  rust-lessons-releases. Publishing is a human decision.
- Finish by setting this task's status to "done" in docs/PLAN.json and committing that. If you cannot finish, set it
  to "blocked" with a one-line reason and commit that, then stop - do not attempt a second approach.

Read docs/RESUME.md, docs/LINEAGE-PLAN.md and docs/PLAN.json first. End your reply with a one-line summary.
"@
    $started = Get-Date
    # the brief goes to a file: ProcessStartInfo in Windows PowerShell 5.1 has Arguments (a string), not
    # ArgumentList, and omp accepts `@file` for a file's contents as the prompt
    $briefPath = Join-Path $logDir ("brief-{0}-{1}.md" -f $task.id, (Get-Date -Format 'yyyyMMdd-HHmmss'))
    [System.IO.File]::WriteAllText($briefPath, $brief, (New-Object System.Text.UTF8Encoding($false)))
    $psi = New-Object System.Diagnostics.ProcessStartInfo
    $psi.FileName = $Omp
    $psi.WorkingDirectory = $tree
    $psi.RedirectStandardOutput = $true
    $psi.RedirectStandardError = $true
    $psi.UseShellExecute = $false
    $psi.Arguments = ('--cwd "{0}" --thinking high @"{1}"' -f $tree, $briefPath)
    $process = [System.Diagnostics.Process]::Start($psi)
    $stdout = $process.StandardOutput.ReadToEndAsync()
    $stderr = $process.StandardError.ReadToEndAsync()
    # heartbeat: "running" and "hung" are different, and only a ticking file can tell them apart
    $heartbeat = Join-Path $logDir 'plan-heartbeat.json'
    $deadline = (Get-Date).AddMinutes($Minutes)
    $timedOut = $false
    while (-not $process.WaitForExit(15000)) {
        $state = [ordered]@{
            task    = $task.id
            branch  = $branch
            tree    = $tree
            started = $started.ToString('yyyy-MM-dd HH:mm:ss')
            elapsed = [int]((Get-Date) - $started).TotalSeconds
            pid     = $process.Id
            alive   = -not $process.HasExited
        }
        [System.IO.File]::WriteAllText($heartbeat, ($state | ConvertTo-Json -Depth 3),
            (New-Object System.Text.UTF8Encoding($false))) 2>$null
        if ((Get-Date) -gt $deadline) { $timedOut = $true; break }
    }
    if ($timedOut) {
        try { $process.Kill($true) } catch { }
        Add-Content -LiteralPath $logPath -Value "TIMEOUT: $($task.id) after $Minutes minutes"
        return "timed out after $Minutes minutes"
    }
    Add-Content -LiteralPath $logPath -Value $stdout.Result
    if ($stderr.Result) { Add-Content -LiteralPath $logPath -Value $stderr.Result }
    Add-Content -LiteralPath $logPath -Value ("exit={0} minutes={1}" -f $process.ExitCode, [int]((Get-Date) - $started).TotalMinutes)
    if ($process.ExitCode -ne 0) { return "session exited $($process.ExitCode)" }
    return $null
}

# master must be clean: an unattended merge on top of somebody's half-finished work is how history starts lying.
# The main tree is the only checkout of the main branch the loop uses, so this check is now the whole guard - a
# worktree elsewhere cannot hold master and therefore cannot block the checkout that no longer happens.
function Assert-CleanMain {
    # The driver's own bookkeeping is not "work in the tree": claiming a task writes docs/PLAN.json, and creating a turn's note writes
    # notes/. Counting those as dirty made every claim abort its own round on 2026-11-07 - two workers claimed, then both exited with
    # 0 tasks. Anything else still has to be clean.
    $dirty = (Invoke-GitQuiet @('status', '--porcelain')) | Where-Object {
        $_ -and $_ -notmatch 'docs[/\\]PLAN\.json' -and $_ -notmatch 'notes[/\\]' -and $_ -notmatch 'logs[/\\]'
    }
    if (($dirty -join '') -ne '') {
        throw "the working tree is not clean: commit or stash before running the plan"
    }
    $branch = (Invoke-GitQuiet @('rev-parse', '--abbrev-ref', 'HEAD')) -join ''
    if ($branch.Trim() -ne $Main) { throw "expected the main tree to be on $Main, found '$($branch.Trim())'" }
    # A rebase refuses an uncommitted tree, and the driver's own bookkeeping - the claim in docs/PLAN.json, the turn's note under notes/ -
    # is written before this runs. Commit exactly those, so the rebase sees a clean tree; anything else dirty already threw above.
    Invoke-Git @('add', 'docs/PLAN.json') | Out-Null
    if (Test-Path -LiteralPath (Join-Path $Repo 'notes')) { Invoke-Git @('add', 'notes') | Out-Null }
    $staged = (Invoke-GitQuiet @('diff', '--cached', '--name-only')) -join ''
    if ($staged.Trim() -ne '') {
        Invoke-Git @('commit', '-m', 'plan: bookkeeping (claim and turn note)') | Out-Null
    }
    Invoke-Git @('fetch', $Remote) | Out-Null
    Invoke-Git @('rebase', "$Remote/$Main") | Out-Null
}

function Test-Branch([string]$Where) {
    # `cargo -C` is nightly-only; a real directory change is the portable way
    Push-Location -LiteralPath $Where
    $previous = $ErrorActionPreference
    # cargo writes "Compiling ..." to stderr, and ErrorActionPreference=Stop turns that into a fatal error
    $ErrorActionPreference = 'Continue'
    try {
        $result = & cargo test 2>&1
        $ok = ($LASTEXITCODE -eq 0)
    } finally {
        $ErrorActionPreference = $previous
        Pop-Location
    }
    return @{ ok = $ok; tail = (($result | Select-Object -Last 6) -join "`n") }
}

$iteration = 0
$noProgress = 0
while (($UntilDone -and $noProgress -lt 3 -and $iteration -lt 99) -or $iteration -lt $Max) {
    if (Test-Path -LiteralPath $stopFile) {
        Write-Output "STOP file present ($stopFile): not starting anything."
        break
    }
    $ledgerNow = Read-Ledger $Repo
    if ($Minutes -le 0) {
        $Minutes = Get-Budget $ledgerNow 20
        Write-Output ("budget from the ledger: {0} minutes" -f $Minutes)
    }
    $pendingCount = @($ledgerNow.tasks | Where-Object { $_.status -eq 'pending' -or $_.status -eq 'running' }).Count
    if ($pendingCount -gt $MaxPending) {
        # workers may add work they discover, but an unbounded queue is a runaway: stop and let a person look
        Write-Output ("stopping: {0} tasks pending, over the cap of {1}" -f $pendingCount, $MaxPending)
        break
    }
    $task = Claim-Task $Worker $refused
    if (-not $task) { Write-Output 'nothing claimable: another worker holds the rest.'; break }
    if (-not $task) { Write-Output 'nothing pending: the plan is complete.'; break }

    $branch = "plan/$($task.id)"
    $tree = Get-TaskTree $task.id
    $stamp = Get-Date -Format 'yyyyMMdd-HHmmss'
    $logPath = Join-Path $logDir ("plan-{0}-{1}.log" -f $stamp, $task.id)
    # One note per turn, created before the worker starts so it has a place to write that cannot collide with any other turn.
    $noteDir = Join-Path $Repo 'notes'
    New-Item -ItemType Directory -Force -Path $noteDir | Out-Null
    $noteName = '{0}-{1}-worker.txt' -f (Get-Date -Format 'yyyy-MM-dd'), $task.id
    $notePath = Join-Path $noteDir $noteName
    if (-not (Test-Path -LiteralPath $notePath)) {
        $noteBody = "# {0} - {1}`n`nTask: {2}`nStarted: {3}`n`n## What this turn changed`n`n## What was measured`n`n## Not done, and why`n`n## What is now fragile`n" -f `
            $task.title, $task.id, $task.id, (Get-Date -Format 'yyyy-MM-dd HH:mm')
        [System.IO.File]::WriteAllText($notePath, $noteBody, (New-Object System.Text.UTF8Encoding($false)))
    }
    Write-Output ("note: {0}" -f $noteName)
    Write-Output ("task {0}: {1}" -f $task.id, $task.title)
    if ($DryRun) {
        Write-Output ("dry run: worktree {0} on {1} from {2}, invoke omp with --cwd there, test there, merge into {3} from {4}, push, remove the tree; log {5}" -f $tree, $branch, $Main, $Main, $Repo, $logPath)
        break
    }

    try {
        # A video batch takes ~14 minutes and writes 200+ MB under out/; a worker racing it would corrupt both. The batch script creates the
    # marker while it runs, so this is a check, not a convention.
    if (Test-Path -LiteralPath (Join-Path $Repo 'logs\video-batch.running')) {
        Write-Output "stopped: a video batch is running (logs\video-batch.running) - release batches and workers take turns."
        break
    }
    Assert-CleanMain
    } catch {
        # the checkout is not what the loop needs: that is the environment, not the task. Stopping without
        # touching the ledger keeps every task runnable once the main tree is clean.
        Write-Output "stopped before starting: $($_.Exception.Message)"
        break
    }

    # A tree an earlier run kept is a decision, not an obstacle. Reclaim it when it holds nothing but a ledger
    # note; refuse THIS task when it holds real work; either way go on, because one kept tree must not cost the
    # whole round (2026-11-06: it started 0 tasks because `worktree add -B` cannot reuse an existing path).
    $verdict = Get-KeptTreeVerdict $task.id $tree
    if (-not $verdict.ok) {
        $refused += $task.id
        $line = "refused: {0} - {1}" -f $task.id, $verdict.reason
        Write-Output $line
        Add-Content -LiteralPath $logPath -Value $line
        $stillPending = @($ledgerNow.tasks | Where-Object { $_.status -eq 'pending' -and $refused -notcontains $_.id })
        if ($stillPending.Count -eq 0) {
            Write-Output 'stopping: every pending task is refused, so a kept tree is in the way of them all'
            break
        }
        continue
    }
    if ($verdict.action -eq 'reclaim') {
        # what the tree held is one note, and the note is worth printing before it is thrown away with the branch
        $noteText = ''
        try {
            $kept = (Read-Ledger $tree).tasks | Where-Object { $_.id -eq $task.id }
            if ($kept -and ($kept.PSObject.Properties.Name -contains 'note')) { $noteText = [string]$kept.note }
        } catch { }
        if ($noteText.Length -gt 160) { $noteText = $noteText.Substring(0, 160) + '...' }
        $line = "reclaimed: {0} - the clean kept tree at {1} (branch {2}) held only the ledger note '{3}'; removed, starting fresh" -f $task.id, $tree, $branch, $noteText
        Write-Output $line
        Add-Content -LiteralPath $logPath -Value $line
        try {
            Remove-TaskTree $tree
        } catch {
            Write-Output ("stopped before starting: the kept tree at {0} could not be removed - {1}" -f $tree, $_.Exception.Message)
            break
        }
    }

    $baseSha = (Invoke-Git @('rev-parse', $Main)) -join ''
    try {
        New-TaskTree $task.id $tree
    } catch {
        Write-Output "stopped before starting: could not create the task tree - $($_.Exception.Message)"
        break
    }

    $failure = $null
    try {
        $failure = Invoke-Task $task $tree $branch $logPath
    } catch {
        # the driver itself failed (a crashed PowerShell, a bad argument): that says nothing about the task, so
        # the loop stops, the tree is kept for inspection and the ledger is left alone
        $keptTrees += $tree
        Write-Output "stopped: the driver failed - $($_.Exception.Message)"
        Write-Output ("kept tree: {0} (branch {1})" -f $tree, $branch)
        break
    }

    $ledgerOnBranch = $null
    try { $ledgerOnBranch = (Read-Ledger $tree).tasks | Where-Object { $_.id -eq $task.id } } catch { }
    $commits = 0
    try { $commits = [int]((Invoke-GitIn $tree @('rev-list', '--count', "$baseSha..HEAD")) -join '') } catch { }

    # a timeout says the clock ran out, not that the work failed: if the branch holds commits and the ledger
    # already records a result, the session finished and was killed on its way out
    if ($failure -and $failure -match 'timed out' -and $commits -gt 0 -and
        $ledgerOnBranch -and $ledgerOnBranch.status -eq 'done') {
        Add-Content -LiteralPath $logPath -Value "timeout ignored: $commits commit(s) and the ledger records done"
        $failure = $null
    }
    if (-not $failure -and (-not $ledgerOnBranch -or $ledgerOnBranch.status -eq 'pending')) {
        $failure = 'the session finished without recording a result in the ledger'
    }
    if (-not $failure -and $commits -eq 0) {
        $failure = 'the session recorded a result but committed nothing'
    }

    if (-not $failure) {
        $test = Test-Branch $tree
        if (-not $test.ok) { $failure = "cargo test failed in the task tree: $($test.tail)" }
    }
    if (-not $failure) {
        $leftover = ((Invoke-GitQuietIn $tree @('status', '--porcelain')) -join "`n").Trim()
        if ($leftover) { $failure = "the session left uncommitted work in the tree: $leftover" }
    }

    $elapsed = 0
    try { $elapsed = [int]((Get-Date) - (Get-Item -LiteralPath $logPath).CreationTime).TotalMinutes } catch { }

    if ($failure -or ($ledgerOnBranch -and $ledgerOnBranch.status -eq 'blocked')) {
        $reason = if ($failure) { $failure } else { $ledgerOnBranch.note }
        Set-TaskStatus $tree $task.id 'blocked' $reason
        Add-Minutes $tree $task.id $elapsed
        try {
            Commit-Ledger $tree ("plan: {0} is blocked - {1}" -f $task.id, $reason)
        } catch {
            Add-Content -LiteralPath $logPath -Value "could not commit the blocked ledger: $($_.Exception.Message)"
        }
        $keptTrees += $tree
        $blockedNow += $task.id
        Write-Output ("blocked: {0} - {1}" -f $task.id, $reason)
        Write-Output ("kept tree: {0} (branch {1}, log {2})" -f $tree, $branch, $logPath)
        break
    }

    # integrate: from the main tree, which never left master
    try {
        # The merge phase - merge, post-merge tests, push, branch delete - is one critical section under a lock: two workers
        # finishing together would otherwise race on master and the loser's commit would be silently lost. Held across the whole
        # phase, not just `git merge`, because the push is what makes the race matter.
        $mergeLock = Enter-PlanLock 'merge'
        Invoke-Git @('merge', '--no-ff', $branch, '-m', ("merge {0}: {1}" -f $branch, $task.title)) | Out-Null
    } catch {
        Invoke-GitQuiet @('merge', '--abort') | Out-Null
        Set-TaskStatus $tree $task.id 'blocked' 'the merge conflicted'
        try { Commit-Ledger $tree ("plan: {0} is blocked - the merge conflicted" -f $task.id) } catch { }
        $keptTrees += $tree
        $blockedNow += $task.id
        Write-Output ("blocked: {0} - merge conflicted" -f $task.id)
        Write-Output ("kept tree: {0} (branch {1})" -f $tree, $branch)
        break
    }

    $postMerge = Test-Branch $Repo
    if (-not $postMerge.ok) {
        Invoke-GitQuiet @('reset', '--hard', $baseSha) | Out-Null
        Set-TaskStatus $tree $task.id 'blocked' "cargo test failed after the merge: $($postMerge.tail)"
        try { Commit-Ledger $tree ("plan: {0} is blocked - the merge broke the build" -f $task.id) } catch { }
        $keptTrees += $tree
        $blockedNow += $task.id
        Write-Output ("blocked: {0} - the merge broke the build; master reset to {1}" -f $task.id, $baseSha)
        Write-Output ("kept tree: {0} (branch {1})" -f $tree, $branch)
        break
    }

    $pushed = $false
    for ($attempt = 1; $attempt -le 3; $attempt++) {
        Invoke-GitQuiet @('fetch', $Remote) | Out-Null
        Invoke-GitQuiet @('rebase', "$Remote/$Main") | Out-Null
        if ((Invoke-GitQuiet @('push', $Remote, $Main)) -notmatch 'rejected|error') {
            $pushed = $true
            break
        }
    }
    if (-not $pushed) {
        Set-TaskStatus $tree $task.id 'blocked' 'the push was rejected three times'
        try { Commit-Ledger $tree ("plan: {0} is blocked - the push was rejected" -f $task.id) } catch { }
        $keptTrees += $tree
        $blockedNow += $task.id
        Write-Output ("blocked: {0} - push rejected (master kept locally at the merge)" -f $task.id)
        Write-Output ("kept tree: {0} (branch {1})" -f $tree, $branch)
        break
    }

    # the work is merged and pushed, so the checkout has done its job: remove it (no --force, so a dirty tree is
    # never deleted) and then the branch it held
    try {
        Remove-TaskTree $tree
    } catch {
        $keptTrees += $tree
        Write-Output ("warning: the merged task tree could not be removed: {0} - {1}" -f $tree, $_.Exception.Message)
    }
    # Not `Invoke-Invoke-GitQuiet`: a doubled prefix from the rename refactor meant every *successful* task ended the driver
    # with exit 1 after its merge - the work landed and the loop still looked crashed (2026-11-06).
    Invoke-GitQuiet @('branch', '-d', $branch) | Out-Null
    Write-Output ("done: {0} ({1} commit(s), merged and pushed, tree removed; log {2})" -f $task.id, $commits, $logPath)
    Exit-PlanLock $mergeLock
    $iteration++
}

# post-loop smoke on master: the loop's own gate is per-task, and a person may have merged in between
$final = Test-Branch $Repo
$report = Join-Path $logDir 'plan-run-done.json'
$summary = [ordered]@{
    finished   = (Get-Date -Format 'yyyy-MM-dd HH:mm:ss')
    repo       = $Repo
    work_root  = $WorkRoot
    tasks      = $iteration
    tests_ok   = $final.ok
    tail       = $final.tail
    ledger     = (Read-Ledger $Repo).tasks.Count
    kept_trees = @($keptTrees)
    refused    = @($refused)
}
[System.IO.File]::WriteAllText($report, ($summary | ConvertTo-Json -Depth 4), (New-Object System.Text.UTF8Encoding($false)))
Release-Claims $Worker

$note = "agent-plan-loop: {0} task(s) done, master tests {1}" -f $iteration, $(if ($final.ok) { 'green' } else { 'RED' })
# a blocked result is committed on the task's branch, not in the main tree, so the run reports the ones it blocked
# itself as well as any an earlier run left in the ledger
$finalLedger = Read-Ledger $Repo
$blockedNames = @($finalLedger.tasks | Where-Object { $_.status -eq 'blocked' } | ForEach-Object { $_.id })
foreach ($id in $blockedNow) { if ($blockedNames -notcontains $id) { $blockedNames += $id } }
if ($blockedNames.Count) { $note = $note + "; blocked: " + ($blockedNames -join ', ') }
if ($refused.Count) { $note = $note + "; refused (a kept tree holds real work): " + ($refused -join ', ') }
if ($keptTrees.Count) { $note = $note + "; kept " + ($keptTrees -join ', ') }
[System.IO.File]::WriteAllText((Join-Path $logDir 'plan-notify.txt'), $note + "`n", (New-Object System.Text.UTF8Encoding($false)))
Write-Output $note
Write-Output ("master smoke: tests_ok={0}" -f $final.ok)
Write-Output ("plan loop finished after {0} task(s)" -f $iteration)
