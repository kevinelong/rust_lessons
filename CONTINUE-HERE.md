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
| Last commit | see `git log`; HEAD carries 0.15.0 with captions |
| Working tree | clean |
| Videos | **21 MP4s + 21 SRT tracks at 0.15.0**, published as `v0.15.0` (46.7 MB, 43 assets) |
| Tests | 110 passing |
| Batch | finished 2026-11-07: `tools/release-videos.ps1 -Sections all -Tag v0.15.0`, 823 s |

**First two things to do after a reboot:**

1. `cd C:/Users/kevin/rust_lessons && git status && cargo test` — the tree should be clean and
   110 tests should pass; if not, something is half-applied.
2. `gh release list --repo kevinelong/rust-lessons-releases | head -3` — expect `v0.15.0` as
   latest. If it is missing or lacks `.srt` assets, rerun
   `powershell -NoProfile -File tools/release-videos.ps1 -Sections all -Tag v0.15.0` (~14 min).

## Next three actions, in order

1. **Send it for review.** `docs/REVIEW.md` to `users.rust-lang.org` (code review category) for
   accuracy on §01–11, and to one past cohort for the beginner pass. Nothing has been sent yet.
2. **Read the five load-bearing sources properly** (Parsons review, Haynes-Magyar, Margulieux,
   Joentausta & Hellas, Kalyuga) and change `skimmed` → `read` in `docs/reading.md` where earned.
   Right now **nothing is marked read**, and two citation errors were already found.
3. **Phase 4, measurement.** Section 14 yields only two retrieval questions, so it cannot be
   scored. Nothing in the course measures a learner, and the course says so.

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

* **Can:** the code compiles, 110 tests pass, CI gates it, the videos regenerate from one
  command, captions align to 0.0 s, `mdbook test` is proven non-vacuous.
* **Cannot:** that anyone learns. No measurement exists, and two factual errors in the prose were
  found only because a reviewer asked a pointed question.
