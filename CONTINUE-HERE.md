# CONTINUE-HERE

Read this first after a reboot, then `docs/STATE.md`.

**Project:** `rust_lessons` — beginning programming taught in Rust, generated from one source
into five renderings (prose, mdBook, slides, notebooks, narrated video) plus practice problems.
Source: <https://github.com/kevinelong/rust_lessons> · Videos: `rust-lessons-releases`.

**Written:** 2026-11-07, while a release batch was running.

## Where the notes live

Everything is inside the repository, under `docs/` — nothing important is in `%TEMP%`.

| File | What it is |
|---|---|
| `CONTINUE-HERE.md` | this page: what is true right now |
| `docs/STATE.md` | the state note: what exists per format level, decisions, defects found, open items |
| `docs/ROADMAP.md` | four phases with observable completion criteria; Phase 4 (measurement) is the only one unbuilt |
| `docs/REVIEW.md` | the packet to hand a reviewer: three bounded tasks |
| `docs/reading.md` | further reading per section, with how deeply each source was read |
| `docs/research/YYYY-MM-DD.md` | daily research notes: read, implies, decided, unknown |
| `docs/video.md`, `docs/compiler-errors.md`, `docs/symbols.md`, `docs/course-plan.md` | supporting detail |
| `docs/diagrams/` | four SVG diagrams + two baked PNGs + `flow.md` (Mermaid sources) |

## State at the moment of writing

| | |
|---|---|
| Last commit | see `git log`; HEAD carries 0.19.0, the checked reading register |
| Working tree | clean |
| Videos | **21 MP4s + 21 SRT tracks at 0.15.0**, published as `v0.15.0` (46.7 MB, 43 assets) |
| Tests | 138: 137 passing, 1 `#[ignore]`d |
| Batch | finished 2026-11-07: `tools/release-videos.ps1 -Sections all -Tag v0.15.0`, 823 s |

**First two things to do after a reboot:**

1. `cd C:/Users/kevin/rust_lessons && git status && cargo test` — the tree should be clean and
   138 tests should pass (137 plus one `#[ignore]`d); if not, something is half-applied.
2. `gh release list --repo kevinelong/rust-lessons-releases | head -3` — expect `v0.15.0` as
   latest. If it is missing or lacks `.srt` assets, rerun
   `powershell -NoProfile -File tools/release-videos.ps1 -Sections all -Tag v0.15.0` (~14 min).

## Next three actions, in order

1. **Send it for review.** `docs/REVIEW.md` to `users.rust-lang.org` (code review category) for
   accuracy on §01–11, and to one past cohort for the beginner pass. Nothing has been sent yet.
2. ~~Read the five load-bearing sources properly~~ — **done in 0.19.0.** Ericson et al. 2022,
   Haynes-Magyar 2025 and Margulieux & Catrambone 2016 are now `read`, each with the pages that
   were looked at and quoted lines in `docs/reading.md`; Joentausta & Hellas and Kalyuga stay
   `skimmed` because both are paywalled (ACM 403, UNSW metadata-only) and an abstract is what
   `skimmed` means. Three citation errors are corrected with their evidence. What is left is
   institutional access, and the one finding that changes a claim: Margulieux & Catrambone found
   a labelled text did **not** help on its own — only text plus a worked example carrying the
   same labels did — so a **Subgoals** block needs a labelled example beside it.
3. **Phase 4, measurement — the surface is built (0.18.0); the human input is not.** Input: every
   section yields at least four retrieval items (0.16.0, 101 over 21, proven non-vacuous) and one
   recorded session exists as data (`fixtures/session-2026-11-07-synthetic.json`, 0.17.0).
   Score: `rust_lessons::scoring` + `cargo run --bin score` print correctness, time to answer and
   retrieval-versus-recognition per section with an overall row, and `tests/session_scoring.rs`
   pins the fixture's numbers exactly. Roadmap phase 4's criterion, quoted: *"Done when a
   stranger's score on section 01 can be recorded without an instructor present."* What remains
   is a session a person actually sat — the binary already accepts one — and only then the
   phase-4 claim about learning, with its n = 1 caveat.

## Environment facts a cold start needs

| Tool | Where | Note |
|---|---|---|
| ffmpeg / ffprobe | `C:\Users\kevin\tools\ffmpeg-9.0.2-essentials_build\bin` | Windows-native; WSL's ffmpeg crashed the WSL service twice |
| Kokoro TTS | `C:\Users\kevin\.ttsvenv\Scripts\python.exe` + models in `C:\Users\kevin\kokoro` | CPU, RTF 0.15, voice `af_heart` |
| Chrome | `C:\Program Files\Google\Chrome\Application\chrome.exe` | headless screenshots replace the MCP browser in the batch |
| mdbook | `C:\Users\kevin\.cargo\bin\mdbook.exe` | `mdbook build book && mdbook test book` |
| Rust kernel | `evcxr_jupyter` **0.19.0** | 0.22.0 does **not** build against rustc 1.96 |

## Traps that have already cost time

* `narrate` **wipes** `out/video/section_NN/`, so re-rendering a section without re-running
  `tts.py` silently produces a video of silent 2.2 s slides. The batch refuses to mux without
  narration; a manual run will not.
* `mdbook test` passed while compiling **nothing** until a `rust` block existed. Verify a check
  is non-vacuous before believing it.
* Per-call model selection is off (`task.perCallModel`), so "poll several AI reviewers" means
  polling one model several times — correlated, not independent.
* `git add -A` in a repo where another process is writing will sweep its files in. Here the only
  writer is `out/`, which is ignored.
* **`cargo test` does not run `cargo doc`.** A doc comment containing `[something]` is parsed as
  an intra-doc link, and CI's `RUSTDOCFLAGS=-D warnings` fails on it — which is how a green local
  `cargo test` shipped a red CI. Run `RUSTDOCFLAGS=-D warnings cargo doc --no-deps` before
  pushing anything that touches `///` or `//!`.

## What this project can and cannot claim

* **Can:** the code compiles, 138 tests pass, CI gates it, the videos regenerate from one
  command, captions align to 0.0 s, `mdbook test` is proven non-vacuous.
* **Cannot:** that anyone learns. No measurement exists, and two factual errors in the prose were
  found only because a reviewer asked a pointed question.
