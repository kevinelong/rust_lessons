# State — rust_lessons

Written 2026-11-05 at v0.6.0, updated 2026-11-07 at v0.18.0; handoff entry point is `CONTINUE-HERE.md` at the repository root. Read this first if you are picking the project up in a new
session; everything here is verifiable from the repository.

## What exists

| Level | Artefact | Status | Evidence |
|---|---|---|---|
| 0 | acceptance criteria | **done** | `cargo test` → 137 tests (136 passing, 1 `#[ignore]`d) |
| 1 | lesson prose | **done** | 21 sections in `src/section_*.rs`; `SECTION_COUNT` is the single bound every generator uses |
| 2 | slides | **done** | 277 HTML slides across 21 sections; all six diagrams embedded as images |
| 3 | notebook | **done** | 21 `.ipynb` in `notebooks/`; 20/20 executed cleanly when sampled, one per section regenerated since |
| 4 | narrated video | **done** | 21 MP4s + 21 SRT at 0.15.0, published as `v0.15.0` (46.7 MB, 43 assets); cues verified to end within 0.0 s of their video |
| — | book (mdBook) | **0.13.0** | 23 chapters in `book/src/`; `mdbook build` + `mdbook test` pass, and the test is proven non-vacuous |
| — | practice | **0.16.0** | 63 generated files: Parsons, faded, retrieval — 21 sections each; retrieval floor is four per section (101 items), asserted by `cargo test` |
| — | scoring | **0.18.0** | `rust_lessons::scoring` scores a session on correctness, time to answer and retrieval-versus-recognition per section, plus an overall row; `cargo run --bin score` prints it; `tests/session_scoring.rs` (7 tests) pins the fixture's numbers exactly |
| — | recorded session | **0.17.0** | `fixtures/session-2026-11-07-synthetic.json` — 8 attempts over sections 01–03, 102.4 s on task, one wrong-then-right pair; read by `rust_lessons::session`, asserted by `tests/session_fixture.rs` (7 tests) |

Published artefacts live outside this repository by design:

* `https://github.com/kevinelong/rust-lessons-releases` — release assets only; `*.mp4` and
  `SHA256SUMS` are git-ignored there.
* `out/` here is git-ignored: 207 MB of generated slides, WAVs and MP4s.

## Decisions, and why

* **Narration drives timing.** Each slide is held for its own measured audio, so the video
  re-derives when the prose changes. A screencast would drift, and re-recording one
  corrected sentence would cost a performance.
* **The course map is authoritative for titles.** Section 4's prose opens with a plain
  sentence and its only `#` heading was a diagram pointer, so a prose-derived title
  produced a video called "Diagram".
* **Beats live in `src/lesson.rs`, not the binary**, so `cargo test` can assert every
  section still has spoken prose. A missing narration is a red test, not a silent video.
* **Kokoro-82M, voice `af_heart`.** Chosen by listening; it beat all four British `bf_*`
  voices on the same sentence. Runs on CPU at RTF 0.15 (~3 s audio per beat-second).
* **SAPI stays as `tts.ps1`** — the dependency-free fallback. Both write the same
  `narration/NNN.wav`, so `mux.ps1` cannot tell which produced the audio.
* **Windows-native ffmpeg, not WSL.** WSL's ffmpeg killed the WSL service twice
  (`Wsl/Service/E_UNEXPECTED`); `mux.ps1` resolves ffmpeg from PATH, `FFMPEG_DIR`, then
  `%USERPROFILE%\tools\ffmpeg-*`.
* **One version constant**, asserted against `Cargo.toml` by a test. 0.1.1 → 0.6.0: every
  bump was a feature, so minor, with patch reset.
* **A session is JSON, read with `serde`/`serde_json`** — the crate's only dependency, both
  already in the local registry cache so `cargo test --offline` builds. Hand-rolling a JSON
  reader to keep the dependency count at zero would have put a parser in the path of every
  score; the schema is versioned (`schema: 1`) so a future session is refused, not half-read.
* **Correctness is derived, not stored.** An attempt carries `answer` and `expected`; whether
  it was right is `answer` compared to `expected`. A fixture that could assert its own verdict
  could contradict its own text, and the wrong-then-right pair would no longer prove anything.
* **A score reports tries, not just items, and both.** A retry is spent time and a second
  verdict, so a section prints `items` and `tries` side by side: section 01 of the fixture has
  three items and four tries, and hiding either number would make the retry invisible or
  double-count the item. Correctness is tries right over tries given; time to answer is mean
  per try, so the wrong first attempt's seconds are not quietly dropped.
* **An untested axis prints `-`, never `0.0%`.** Zero of zero is unknown. Section 02 asks no
  Parsons problem, so its recognition cell is `-` and `Tally::accuracy` returns `None`; a
  silent zero would read as a failure the learner never had a chance to commit.

## Defects found by building it (all fixed)

| Symptom | Cause | Fix |
|---|---|---|
| 4 beats for a 10-beat lesson | reading only `//!`, not `///` | `doc_block` reads both, stops at `#[cfg(test)]` |
| `libx264`: "height not divisible by 2" | slides capture at 2000×1125 (DPR 1.25) | mux scales to 1920×1080 |
| PowerShell parse error | here-strings cannot carry content on the header line | plain quoted strings |
| A red mark after the last glyph | ClearType subpixel fringing, not content | `-webkit-font-smoothing: antialiased` |
| Video called "Diagram" | title derived from the first `#` heading | titles from `COURSE_MAP`/`TRACK_B` |
| `cargo run` ambiguous | two binaries | `default-run = "rust_lessons"` |

## Commands

```bash
cargo test                                    # the grader
cargo test --test session_fixture              # the recorded session loads and its numbers hold
cargo test --test session_scoring              # the fixture scores to the pinned numbers
cargo run --bin score                          # score the fixture: 3 axes per section + overall
cargo run --bin score -- path/to/session.json  # score any session the schema accepts
cargo run                                     # course map
cargo run -- 7                                # one section's lesson text
cargo run --example 07_vectors                # the lesson, running
cargo run --bin narrate -- 7 --voice bf_emma  # slides + narration text + scripts
powershell -NoProfile -File out/video/section_07/tts.ps1     # SAPI fallback
C:\Users\kevin\.ttsvenv\Scripts\python.exe out/video/section_07/tts.py   # Kokoro
powershell -NoProfile -File out/video/section_07/mux.ps1     # MP4
```

The screenshot pass between `tts` and `mux` is the one manual step: serve `out/video` over
HTTP and screenshot each `slide_NNN.html` at 1600×900 into `slides/`.

## Release batch

`tools/release-videos.ps1` regenerates and publishes in one command:

```text
powershell -NoProfile -File tools/release-videos.ps1 -Sections all -Tag v0.15.0 -Notes "..."
powershell -NoProfile -File tools/release-videos.ps1 -Sections 7,8 -SkipPublish
```

A full run is ~12 minutes (21 sections: narration, 277 screenshots, mux). It refuses to mux a
section with no narration — the failure that silently produced 2.2 s slides twice before it
was caught by comparing durations.

## AI review: a lint, not a jury

Two independent subagent reviews agreed on all five claims — because per-call model selection is
off, so both were the *same* model wearing different hats. Correlated reviewers do not average
into truth. Use them for grounded, checkable findings (a claim to verify against the compiler, a
socket to test, a sentence to quote), never as a crowd. One review produced the §04 `'static`
correction and the 404→405 fix; its RST prediction did not reproduce.

## Review

`docs/REVIEW.md` is the packet to hand a volunteer: three bounded tasks, the five claims most
likely to be wrong, and the rule that "this would mislead a beginner because…" is the useful
report. Nothing has been sent to anyone yet — the venues and what each is good for are recorded
here rather than in a plan: past cohorts for beginner reaction, `users.rust-lang.org` code review
for idiom accuracy, Rust-Edu for the pedagogy claims, WebAIM-class communities for accessibility.

## Sourcing

`docs/reading.md` carries every section's primary source and an alternate take, each labelled
with how deeply it was read. **Nothing is yet marked `read`** — the register says so in its own
last line, because a claim resting on a skimmed paper is one paper-deep, and a claim resting on
a named one is a guess.

Two citation errors were found and corrected on 2026-11-06 (a wrong year, an overstated claim);
see `docs/research/2026-11-06.md`.

## Research and plan

* `docs/research/YYYY-MM-DD.md` — daily notes: what was read, what it implies, what was
  decided, what is still unknown. First entry: 2026-11-06.
* `docs/ROADMAP.md` — four phases with observable completion criteria.

## Open items

1. ~~Notebook tier~~ — built. `cargo run --bin notebook` writes one notebook per section;
   cells load the crate via `:dep` and print real results. `evcxr_jupyter` **0.22.0 does not
   build** against rustc 1.96 (its pinned `ra_ap_span` fails), so the kernel is **0.19.0**.
   Execute with nbclient: `nbclient` + `jupyter-client`, kernel name `rust`.
2. **YouTube** — needs OAuth (Data API v3) credentials this repo does not carry. The MP4s
   are upload-ready; a title/description sidecar is all that is missing.
3. ~~Diagrams missing from slides and video~~ — fixed in 0.8.0. Diagram beats embed the
   artwork (base64) with the surrounding sentence as a caption. The two Mermaid diagrams
   are baked to PNG beside the SVGs (`flow-iterator-laziness.png`, `flow-question-mark.png`)
   by rendering `docs/diagrams/flow.md` through mermaid 10 in Chromium with
   `useMaxWidth: false` and screenshotting the card — Mermaid cannot be embedded as a file.
4. **WSL image compaction** — the 47 GB `ext4.vhdx` needs `--allow-unsafe` or an elevated
   `diskpart compact vdisk`; left to Kevin.
5. **Measurement** — surface built in 0.18.0, no human session yet. Practice artefacts exist
   (Parsons, faded, retrieval), every section holds at least four retrieval items (0.16.0), one
   **recorded session as data** exists (0.17.0, `fixtures/session-2026-11-07-synthetic.json`),
   and `rust_lessons::scoring` now scores that session on the three axes. Roadmap phase 4's
   completion criterion, quoted: *"Parsons + retrieval artefacts are scoreable; a section ends
   with 'you can now do X' and a task that produces a number. Done when a stranger's score on
   section 01 can be recorded without an instructor present."* The input and the arithmetic
   are both in place; the fixture is synthetic — written, not sat — so the table proves the
   reader and the arithmetic, never a person's learning. Still missing: a session a person
   actually sat, which is the input `cargo run --bin score` already accepts, and then the
   phase-4 claim (item 11) with its n = 1 caveat.
6. ~~Section 14 has two retrieval questions~~ — **fixed in 0.16.0.** Sections 12, 14 and 17
   were given the sentences they were missing (a URL's four fields and default ports, DNS
   record types, private ranges; the narrowing range; cloning as the third consequence of
   ownership, and the compiler error a move produces). Every section now yields four or five
   items, 101 in total, and `every_section_has_enough_retrieval_items_to_be_scored` fails if
   any section drops below four. The assertion was proven non-vacuous by deleting one prose
   sentence from section 05, which failed it with "section 05 yields only 3 retrieval items".
7. **Completion reserve** — the 1M-token window is eaten by a ~307k completion reservation;
   no `maxTokens` key exists in `config.yml`, so this needs the provider's per-model schema
   confirmed before changing.

## Environment facts worth keeping

* RTX 5070, 12 GB VRAM; 63 GB RAM; C: has ~51 GiB free after clearing 32.6 GB of
  `iimage-frames-*` from `%TEMP%`.
* ffmpeg 9.0.2 (gyan.dev essentials) at `C:\Users\kevin\tools\ffmpeg-9.0.2-essentials_build`.
* Kokoro models at `C:\Users\kevin\kokoro` (310 MB + 27 MB); venv at `C:\Users\kevin\.ttsvenv`.
* WSL 2.7.3 with no running distribution; its ffmpeg is *not* used by anything here.
