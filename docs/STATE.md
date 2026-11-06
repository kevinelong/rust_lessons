# State — rust_lessons

Written 2026-11-05, at v0.6.0. Read this first if you are picking the project up in a new
session; everything here is verifiable from the repository.

## What exists

| Level | Artefact | Status | Evidence |
|---|---|---|---|
| 0 | acceptance criteria | **done** | `cargo test` → 85 tests (81 unit, 4 integration), 1 `#[ignore]`d |
| 1 | lesson prose | **done** | 62 tracked files; 20 sections in `src/section_*.rs` |
| 2 | slides | **done** | `cargo run --bin narrate -- N` → 261 HTML slides; all six diagrams embedded as images |
| 3 | notebook | **done** | 20 `.ipynb` in `notebooks/`; **20/20 execute** against the Rust kernel |
| 4 | narrated video | **done** | 20 MP4s, 26:59, 34.8 MB, published as `rust-lessons-releases` v0.8.0 |
| — | practice | **0.10.0** | 60 generated files: Parsons, faded, retrieval — 20 sections each |

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
5. **Measurement** — practice artefacts exist (Parsons, faded, retrieval) but nothing
   records a score. Section 14 yields only two retrieval questions; every section should
   reach five before a learner can be assessed on it.
6. **Completion reserve** — the 1M-token window is eaten by a ~307k completion reservation;
   no `maxTokens` key exists in `config.yml`, so this needs the provider's per-model schema
   confirmed before changing.

## Environment facts worth keeping

* RTX 5070, 12 GB VRAM; 63 GB RAM; C: has ~51 GiB free after clearing 32.6 GB of
  `iimage-frames-*` from `%TEMP%`.
* ffmpeg 9.0.2 (gyan.dev essentials) at `C:\Users\kevin\tools\ffmpeg-9.0.2-essentials_build`.
* Kokoro models at `C:\Users\kevin\kokoro` (310 MB + 27 MB); venv at `C:\Users\kevin\.ttsvenv`.
* WSL 2.7.3 with no running distribution; its ffmpeg is *not* used by anything here.
