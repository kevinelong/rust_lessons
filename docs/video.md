# Formats, and the narrated video

The course is one source with four derived renderings. The rule: **nothing is
maintained twice.** Every artefact below is generated from the lesson modules or is the
lesson modules.

| Level | Artefact | Built by | Interactive |
|---|---|---|---|
| 0 | acceptance criteria | `cargo test` (85 tests) | no, it is the grader |
| 1 | lesson prose | rustdoc `//!` + `///`, `docs/` | read |
| 2 | slides | `cargo run --bin narrate -- N` → `out/video/section_NN/slide_*.html` | click |
| 3 | notebook | `evcxr_jupyter` (Rust kernel) — not wired up yet | edit and run |
| 4 | narrated video | the same `narrate` output, muxed | none |

Levels 2 and 4 are implemented and verified. Level 3 is a documented option, not a
promise: `evcxr_jupyter` 0.22.0 registers a real Rust kernel, but this repository has no
`.ipynb` yet, so nothing here should be read as claiming one exists.

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
* `tts.ps1` — `System.Speech` synthesis, `Rate = -1`, one WAV per beat
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

## Verified output

Section 1: `1920×1080`, h264 + aac, **107.2 s**, 2.05 MB, ten beats, ten narration
WAVs. Slides advance on narration boundaries, not on a fixed timer.

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
* Two of the six diagrams are Mermaid (`docs/diagrams/flow.md`) and therefore are **not**
  in the video yet; the four SVGs are.
