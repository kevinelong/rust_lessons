# Formats, and the narrated video

The course is one source with four derived renderings. The rule: **nothing is
maintained twice.** Every artefact below is generated from the lesson modules or is the
lesson modules.

| Level | Artefact | Built by | Interactive |
|---|---|---|---|
| 0 | acceptance criteria | `cargo test` (85 tests) | no, it is the grader |
| 1 | lesson prose | rustdoc `//!` + `///`, `docs/` | read |
| 2 | slides | `cargo run --bin narrate -- N` → `out/video/section_NN/slide_*.html` | click |
| 3 | notebook | `cargo run --bin notebook` → `notebooks/section_NN.ipynb` | edit and run |
| 4 | narrated video | the same `narrate` output, muxed | none |

All four are implemented and verified: 261 slides, 20 videos, and 20 notebooks that execute
against the real Rust kernel (20/20 clean, outputs confirmed — e.g. section 11 prints the
seeded dice histogram).

## Building a section's video

```bash
cargo run --bin narrate -- 1                    # beats → slides + narration text + scripts
powershell -NoProfile -File out/video/section_01/tts.ps1    # Windows SAPI → one WAV per beat
# screenshot each out/video/section_01/slide_NNN.html into out/video/section_01/slides/
powershell -NoProfile -File out/video/section_01/mux.ps1    # ffmpeg → section_01.mp4
```

`narrate` writes everything a human would otherwise assemble by hand:

* `beats.json` — the parse, so any other renderer can consume it
* `slide_NNN.html` — one self-contained dark slide per beat, 16:9, grayscale antialiasing
* `narration/NNN.txt` — the words, markdown stripped
* `tts.py` — **Kokoro-82M** synthesis (Apache-2.0), one WAV per beat — the preferred path
* `tts.ps1` — `System.Speech` synthesis, `Rate = -1` — the fallback with no dependencies
* `mux.ps1` — duration-driven assembly (below)

### Why narration drives the timing

Each slide is held for exactly the length of its own audio, measured with `ffprobe`. A
screencast's timing drifts from the words, and re-recording one corrected sentence costs
a performance; here the words are authoritative and the video re-derives itself whenever
the prose changes. Beats with nothing to say (code, tables, diagram pointers) are given
`SILENT_HOLD` seconds of generated silence.

Two constraints found by building it:

* **h264 needs even dimensions.** Captured slides come back 2000×1125 at
  `devicePixelRatio` 1.25, and libx264 refuses an odd height. The mux scales to
  1920×1080 with lanczos, which also makes the output consistent across machines.
* **ffmpeg must be Windows-native.** WSL's ffmpeg crashed this machine's WSL service
  twice (`Wsl/Service/E_UNEXPECTED`), so `mux.ps1` discovers `ffmpeg.exe` from `PATH`,
  then `$env:FFMPEG_DIR`, then `%USERPROFILE%\tools\ffmpeg-*\bin`.

## The voice

`af_heart`, chosen by listening. Kokoro's voice names encode language and gender —
`af_` American female, `bf_` British female, `am_`/`bm_` the male equivalents — and on the
same sentence `af_heart` beat all four British female voices (`bf_emma`, `bf_isabella`,
`bf_alice`, `bf_lily`).

    cargo run --bin narrate -- 1 --voice bf_emma --speed 0.95

Kokoro needs `kokoro-onnx` plus two model files (310 MB + 27 MB, `KOKORO_DIR`, default
`%USERPROFILE%\kokoro`). It runs on CPU here at RTF 0.15 — about 3 s of audio per beat
per second of compute — so a twelve-beat section narrates in 15 s. The CUDA path exists
but needs ~2–3 GB of onnxruntime-gpu and cuDNN, which this machine did not have free.

SAPI remains as `tts.ps1` for anyone without the Python stack: same input directory, same
output filenames, so `mux.ps1` cannot tell which produced the audio.

## Verified output

`cargo run --bin narrate -- N` for N in 1..20, then `tts.py`, a screenshot pass, then
`mux.ps1` — 0 mux failures across the set.

All twenty sections, 26:59 in total, 34.5 MB of video: 261 slides and 249 narration WAVs.
The four sections that also exist in the SAPI release run 20–25 % shorter with Kokoro
(`section_01` 1:25 against 1:47). Per-section lengths are in the release notes of
`rust-lessons-releases`.

`1920×1080`, h264 + aac at 24 kHz mono, max volume −2.0 dB, slides advancing on narration
boundaries rather than a fixed timer. Kokoro delivers the same words in roughly 20–25 % less
time than the SAPI voice.

## Publishing

The video is a build artefact, not source: `out/` is git-ignored, and a 2 MB MP4 (let
alone 20 of them) does not belong in git — the same mistake as the 45 MB PowerPoint in
`python_lessons`, which is four TIFFs in a zip.

* **GitHub:** attach each `section_NN.mp4` to a release, following the
  `<project>-releases` convention used for binaries.
* **YouTube:** upload needs OAuth credentials (Data API v3) which this repository does
  not carry; the MP4 plus a title/description sidecar is the hand-off point.

## Known limits

* Narration is synthetic and flat. `Zira` and `David` are the only installed en-US
  voices; a human track would need a different `tts.ps1` step and nothing else.
* Slides are static — no cursor, no typing animation. That is a deliberate consequence
  of narration-driven timing: an animation would have to be re-timed per voice.
* All six diagrams are in the slides and the video: the four SVGs are embedded directly and
  the two Mermaid ones are baked to PNG first (see `docs/STATE.md` for the bake recipe).
