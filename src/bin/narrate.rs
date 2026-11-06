//! `narrate` — turn one lesson into the raw material for a narrated video.
//!
//! The lesson module's `//!` prose is the only source. This binary splits it into
//! **beats** (a paragraph, a code block, a table, a bullet list) and writes, under
//! `out/video/section_NN/`:
//!
//! * `beats.json` — the beats, for anything that wants to re-render them
//! * `slide_NNN.html` — one self-contained dark slide per beat, for screenshotting
//! * `narration/NNN.txt` — the words to speak for the beats that are prose
//! * `tts.ps1` — Windows SAPI text-to-speech, one WAV per narration file
//! * `mux.sh` — WSL ffmpeg: hold each slide for its own narration length, concat, mux
//!
//! Usage: `cargo run --bin narrate -- 1`
//!
//! Why generate rather than record: a screencast's timing drifts from the words, and
//! re-recording a fixed sentence costs a performance. Here the narration is
//! authoritative — each slide is held for exactly as long as its own audio — so the
//! video re-derives itself whenever the lesson text changes.

use std::fs;
use std::path::PathBuf;

use rust_lessons::lesson::{self, Beat, Kind};

const SILENT_HOLD: f64 = 2.2;

fn main() -> std::process::ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some(n) = args.first().and_then(|a| a.parse::<u8>().ok()) else {
        eprintln!("usage: narrate <section 1-20> [--voice NAME] [--speed F]");
        return std::process::ExitCode::from(2);
    };
    if !(1..=rust_lessons::SECTION_COUNT).contains(&n) {
        eprintln!("no such section: {n}");
        return std::process::ExitCode::from(2);
    }

    let module = match lesson::module_path(n) {
        Some(p) => p,
        None => {
            eprintln!("cannot find src/section_{n:02}_*.rs");
            return std::process::ExitCode::from(1);
        }
    };
    let voice = arg_value(&args, "--voice").unwrap_or_else(|| String::from(DEFAULT_VOICE));
    let speed: f64 = arg_value(&args, "--speed")
        .and_then(|v| v.parse().ok())
        .unwrap_or(1.0);

    let beats = match lesson::beats(n) {
        Some(b) => b,
        None => {
            eprintln!("{} has no lesson prose to narrate", module.display());
            return std::process::ExitCode::from(1);
        }
    };
    let title = chapter_title(&beats, n);

    let out = PathBuf::from(format!("out/video/section_{n:02}"));
    let _ = fs::remove_dir_all(&out);
    fs::create_dir_all(out.join("narration")).expect("create output directory");

    fs::write(out.join("beats.json"), beats_json(&beats)).expect("write beats.json");
    for (i, beat) in beats.iter().enumerate() {
        fs::write(
            out.join(format!("slide_{i:03}.html")),
            slide_html(i, &beats, &title, n),
        )
        .expect("write slide");
        if lesson::needs_narration(beat) {
            fs::write(
                out.join(format!("narration/{i:03}.txt")),
                lesson::speakable(&beat.text),
            )
            .expect("write narration");
        }
    }
    fs::write(out.join("tts.ps1"), tts_ps1()).expect("write tts.ps1");
    fs::write(out.join("tts.py"), tts_py(&voice, speed)).expect("write tts.py");
    fs::write(out.join("mux.ps1"), mux_ps1(&beats, n)).expect("write mux.ps1");

    println!("section {n:02}: {title}");
    println!("  beats: {}", beats.len());
    println!(
        "  narrated: {}",
        beats.iter().filter(|b| lesson::needs_narration(b)).count()
    );
    println!("  output: {}", out.display());
    println!();
    println!("next:");
    println!("  {0} {1}/tts.py          # Kokoro voice \"{2}\" (needs kokoro-onnx)", venv_python(), out.display(), voice);
    println!("  powershell -NoProfile -File {}/tts.ps1            # SAPI fallback, no dependencies", out.display());
    println!("  (screenshot each slide_NNN.html into slides/, then)");
    println!("  powershell -NoProfile -File {}/mux.ps1", out.display());
    std::process::ExitCode::SUCCESS
}

fn chapter_title(beats: &[Beat], n: u8) -> String {
    // The course map is authoritative: the prose may begin with a plain sentence (or,
    // in section 4, with a "# Diagram" heading), and neither is the section's name.
    let from_map = rust_lessons::COURSE_MAP
        .iter()
        .chain(rust_lessons::TRACK_B.iter())
        .find(|(num, _)| *num == format!("{n:02}"))
        .map(|(_, t)| t.to_string());
    from_map
        .or_else(|| {
            beats
                .iter()
                .find(|b| b.kind == Kind::Heading)
                .map(|b| b.text.trim_start_matches('#').trim().to_string())
        })
        .unwrap_or_else(|| format!("Section {n}"))
}

fn beats_json(beats: &[Beat]) -> String {
    let mut s = String::from("[\n");
    for (i, b) in beats.iter().enumerate() {
        s.push_str(&format!(
            "  {{\"i\": {i}, \"kind\": \"{}\", \"text\": \"{}\"}}{}\n",
            match b.kind {
                Kind::Heading => "heading",
                Kind::Prose => "prose",
                Kind::Code => "code",
                Kind::Bullets => "bullets",
                Kind::Table => "table",
                Kind::Diagram => "diagram",
            },
            json_escape(&b.text),
            if i + 1 == beats.len() { "" } else { "," }
        ));
    }
    s.push_str("]\n");
    s
}

fn json_escape(s: &str) -> String {
    let mut out = String::new();
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push(' '),
            c => out.push(c),
        }
    }
    out
}

/// Minimal inline markdown: escape HTML first, then code spans and bold.
fn inline(text: &str) -> String {
    let escaped = text
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;");
    let mut out = String::new();
    let mut in_code = false;
    for part in escaped.split('`') {
        if in_code {
            out.push_str("<code>");
            out.push_str(part);
            out.push_str("</code>");
        } else {
            out.push_str(part);
        }
        in_code = !in_code;
    }
    out.replace("**", "")
}

/// The words around a diagram reference, with the file paths removed.
fn diagram_caption(text: &str) -> String {
    let mut kept: Vec<String> = Vec::new();
    for word in text.split_whitespace() {
        if word.contains("docs/diagrams/") {
            continue;
        }
        kept.push(word.trim_matches('`').to_string());
    }
    let caption = kept.join(" ").trim_matches(|c: char| c == ';' || c.is_whitespace()).to_string();
    if caption.is_empty() {
        String::new()
    } else {
        format!("<p class=\"caption\">{}</p>", inline(&caption))
    }
}

/// Turn a diagram beat's text into `<img>` tags with the artwork inlined.
///
/// The lesson says which diagram it means, either as a file (`docs/diagrams/ownership.svg`)
/// or as a Mermaid anchor in `docs/diagrams/flow.md`. Mermaid cannot be embedded as a file,
/// so its two diagrams are baked to PNG beside the SVGs; `docs/STATE.md` records how.
fn diagram_images(text: &str) -> Option<String> {
    let mut found: Vec<&str> = Vec::new();
    for token in text.split(['`', ' ', '\n', ',']) {
        let token = token.trim();
        if let Some(rest) = token.strip_prefix("docs/diagrams/") {
            if let Some(name) = rest.strip_suffix(".svg") {
                found.push(Box::leak(format!("docs/diagrams/{name}.svg").into_boxed_str()));
            } else if rest.starts_with("flow.md") {
                if rest.contains("iterator-laziness") {
                    found.push("docs/diagrams/flow-iterator-laziness.png");
                } else if rest.contains("propagation") {
                    found.push("docs/diagrams/flow-question-mark.png");
                }
            }
        }
    }
    found.dedup();
    if found.is_empty() {
        return None;
    }
    let mut out = String::new();
    for path in found {
        let Ok(bytes) = std::fs::read(path) else {
            continue;
        };
        let mime = if path.ends_with(".svg") {
            "image/svg+xml"
        } else {
            "image/png"
        };
        out.push_str(&format!(
            "<img class=\"diagram\" alt=\"{}\" src=\"data:{};base64,{}\">",
            path.rsplit('/').next().unwrap_or("diagram"),
            mime,
            base64(&bytes)
        ));
    }
    if out.is_empty() {
        None
    } else {
        Some(out)
    }
}

/// Standard base64, so embedding an image needs no dependency.
fn base64(bytes: &[u8]) -> String {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let b = [
            chunk[0],
            *chunk.get(1).unwrap_or(&0),
            *chunk.get(2).unwrap_or(&0),
        ];
        let n = ((b[0] as u32) << 16) | ((b[1] as u32) << 8) | b[2] as u32;
        out.push(ALPHABET[(n >> 18) as usize & 63] as char);
        out.push(ALPHABET[(n >> 12) as usize & 63] as char);
        out.push(if chunk.len() > 1 {
            ALPHABET[(n >> 6) as usize & 63] as char
        } else {
            '='
        });
        out.push(if chunk.len() > 2 {
            ALPHABET[n as usize & 63] as char
        } else {
            '='
        });
    }
    out
}

fn slide_html(index: usize, beats: &[Beat], title: &str, section: u8) -> String {
    let beat = &beats[index];
    let is_title = index == 0 && beat.kind == Kind::Prose && beat.text.trim_start().starts_with("Section");
    let body = if is_title {
        let rest = beat
            .text
            .trim_start()
            .split_once('—')
            .map(|(_, tail)| tail.trim().to_string())
            .unwrap_or_else(|| beat.text.clone());
        format!(
            "<p class=\"kicker\">rust_lessons {ver} · section {n:02}</p><h1 class=\"title\">{title}</h1>",
            ver = rust_lessons::VERSION,
            n = section,
            title = inline(&rest)
        )
    } else { match beat.kind {
        Kind::Code => format!("<pre class=\"code\">{}</pre>", inline(&strip_fence(&beat.text))),
        Kind::Table => format!("<pre class=\"mono\">{}</pre>", inline(&beat.text)),
        Kind::Diagram => match diagram_images(&beat.text) {
            Some(imgs) => format!("{imgs}{}", diagram_caption(&beat.text)),
            None => format!("<pre class=\"mono\">{}</pre>", inline(&beat.text)),
        },
        Kind::Bullets => format!("<ul>{}</ul>", bullet_items(&beat.text)),
        _ => format!("<p>{}</p>", inline(&beat.text)),
    } };
    format!(
        r#"<!doctype html>
<html lang="en"><head><meta charset="utf-8"><title>{title} — beat {index}</title>
<style>
  :root {{ color-scheme: dark; }}
  html,body {{ margin:0; height:100%; }}
  body {{ background:#0d1117; color:#e6edf3; font-family: ui-monospace, Consolas, monospace;
         display:flex; flex-direction:column; justify-content:center; padding:0 96px;
         -webkit-font-smoothing: antialiased; text-rendering: geometricPrecision; }}
  h1.title {{ font-family: system-ui, sans-serif; font-size:58px; font-weight:680; line-height:1.2;
              margin:0; max-width:22em; }}
  p.kicker {{ font-family: system-ui, sans-serif; color:#8b949e; font-size:22px; margin:0 0 16px; }}
  header {{ font-family: system-ui, sans-serif; color:#8b949e; font-size:19px; margin-bottom:30px;
            display:flex; justify-content:space-between; }}
  h2 {{ font-family: system-ui, sans-serif; font-size:34px; margin:0 0 22px; font-weight:640; }}
  p {{ font-size:24px; line-height:1.55; margin:0; max-width:26em; }}
  ul {{ font-size:22px; line-height:1.6; margin:0; padding-left:1.1em; }}
  li {{ margin-bottom:.35em; }}
  pre.code {{ font-size:21px; line-height:1.5; margin:0; padding:22px 26px; background:#161b22;
              border:1px solid #30363d; border-radius:10px; white-space:pre-wrap; }}
  pre.mono {{ font-size:16px; line-height:1.45; margin:0; color:#c9d1d9; white-space:pre-wrap; }}
  p.caption {{ font-size:19px; color:#8b949e; margin:14px 0 0; max-width:34em; }}
  img.diagram {{ display:block; max-width:100%; max-height:560px; margin:10px 0; border-radius:8px; background:#ffffff; }}
  code {{ color:#79c0ff; }}
  footer {{ position:fixed; bottom:22px; right:28px; font-family: system-ui, sans-serif;
            color:#57606a; font-size:15px; }}
</style></head>
<body>
  <header><span>{title}</span><span>rust_lessons {version}</span></header>
  {heading}
  {body}
  <footer>{section:02}/{index}</footer>
</body></html>
"#,
        title = inline(title),
        version = rust_lessons::VERSION,
        heading = if beat.kind == Kind::Heading {
            String::new()
        } else {
            String::new()
        },
        body = body,
        section = section,
        index = index + 1
    )
}

fn strip_fence(text: &str) -> String {
    let mut lines: Vec<&str> = text.lines().collect();
    if lines.first().map(|l| l.trim_start().starts_with("```")) == Some(true) {
        lines.remove(0);
    }
    if lines.last().map(|l| l.trim() == "```") == Some(true) {
        lines.pop();
    }
    lines.join("\n")
}

fn bullet_items(text: &str) -> String {
    text.lines()
        .filter(|l| l.trim_start().starts_with("- ") || l.trim_start().starts_with("* "))
        .map(|l| format!("<li>{}</li>", inline(l.trim_start()[2..].trim())))
        .collect()
}

/// The voice this course narrates with.
///
/// Chosen by listening: `af_heart` beat all four British `bf_*` voices on the same
/// sentence. Kokoro's voice names encode language and gender — `af_` American female,
/// `bf_` British female, `am_`/`bm_` the male equivalents.
const DEFAULT_VOICE: &str = "af_heart";

/// `--voice NAME` or `--speed 0.9` from the arguments.
fn arg_value(args: &[String], flag: &str) -> Option<String> {
    args.iter()
        .position(|a| a == flag)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

/// The interpreter that has `kokoro-onnx` installed, if the conventional venv exists.
fn venv_python() -> String {
    let home = std::env::var("USERPROFILE").unwrap_or_default();
    let candidate = format!("{home}\\.ttsvenv\\Scripts\\python.exe");
    if std::path::Path::new(&candidate).exists() {
        candidate
    } else {
        String::from("python")
    }
}

/// A Python narration pass through Kokoro-82M (Apache-2.0).
///
/// The sibling of `tts.ps1`: same input directory, same output filenames, so `mux.ps1`
/// cannot tell which one produced the audio. Kokoro needs `kokoro-onnx` and two model
/// files; SAPI needs nothing. Neither is required by `cargo test`.
fn tts_py(voice: &str, speed: f64) -> String {
    format!(
        r#"#!/usr/bin/env python3
"""Narrate every narration/*.txt with Kokoro-82M.

Same contract as tts.ps1: reads narration/NNN.txt, writes narration/NNN.wav, so
mux.ps1 works either way. Needs kokoro-onnx plus the model files:

    python -m pip install kokoro-onnx soundfile
    set KOKORO_DIR  (defaults to %USERPROFILE%\kokoro)
      kokoro-v1.0.onnx   310 MB
      voices-v1.0.bin     27 MB

Voice: {voice}  (af_ = American female, bf_ = British female, per Kokoro's naming)
"""
import glob
import os
import sys

import soundfile as sf
from kokoro_onnx import Kokoro

HERE = os.path.dirname(os.path.abspath(__file__))
MODELS = os.environ.get("KOKORO_DIR", os.path.join(os.path.expanduser("~"), "kokoro"))
VOICE = os.environ.get("KOKORO_VOICE", "{voice}")
SPEED = float(os.environ.get("KOKORO_SPEED", "{speed}"))
LANG = "en-us" if VOICE.startswith("a") else "en-gb"

kokoro = Kokoro(os.path.join(MODELS, "kokoro-v1.0.onnx"), os.path.join(MODELS, "voices-v1.0.bin"))

paths = sorted(glob.glob(os.path.join(HERE, "narration", "*.txt")))
if not paths:
    sys.exit("no narration/*.txt — run narrate first")

for path in paths:
    text = open(path, encoding="utf-8").read().strip()
    if not text:
        continue
    out = os.path.splitext(path)[0] + ".wav"
    samples, rate = kokoro.create(text, voice=VOICE, speed=SPEED, lang=LANG)
    sf.write(out, samples, rate)
    print("narrated", os.path.basename(out), f"{{len(samples) / rate:.2f}}s", f"voice={{VOICE}}")
"#
    )
}

fn tts_ps1() -> String {
    r#"$ErrorActionPreference = "Stop"
Add-Type -AssemblyName System.Speech
$synth = New-Object System.Speech.Synthesis.SpeechSynthesizer
$voice = $synth.GetInstalledVoices() | Where-Object { $_.VoiceInfo.Culture.Name -eq "en-US" } | Select-Object -First 1
if ($voice) { $synth.SelectVoice($voice.VoiceInfo.Name) }
$synth.Rate = -1
$here = Split-Path -Parent $MyInvocation.MyCommand.Path
foreach ($f in Get-ChildItem -Path (Join-Path $here "narration") -Filter *.txt | Sort-Object Name) {
  $wav = Join-Path $here ("narration\" + $f.BaseName + ".wav")
  $text = Get-Content -Raw -Path $f.FullName
  $synth.SetOutputToWaveFile($wav)
  $synth.Speak($text)
  $synth.SetOutputToNull()
  Write-Host ("narrated " + $f.BaseName)
}
$synth.Dispose()
"#
    .to_string()
}

/// A PowerShell script that assembles the video on Windows.
///
/// WSL is deliberately not used: the machine's WSL service failed twice under ffmpeg
/// here, and a course that only builds its video on a healthy WSL is a course whose
/// video does not build. ffmpeg is discovered from PATH, then `$env:FFMPEG_DIR`, then
/// `C:\Users\kevin\tools\ffmpeg-*\bin`.
fn mux_ps1(beats: &[Beat], section: u8) -> String {
    let mut s = String::new();
    s.push_str("$ErrorActionPreference = \"Stop\"\n");
    s.push_str("# Assemble the narrated video. Requires ffmpeg + ffprobe and the\n");
    s.push_str("# screenshots in slides/slide_NNN.png, named to match slide_NNN.html.\n");
    s.push_str("$here = Split-Path -Parent $MyInvocation.MyCommand.Path\n");
    s.push_str("Set-Location $here\n");
    s.push_str("$ff = (Get-Command ffmpeg -ErrorAction SilentlyContinue).Source\n");
    s.push_str("if (-not $ff -and $env:FFMPEG_DIR) { $ff = Join-Path $env:FFMPEG_DIR 'ffmpeg.exe' }\n");
    s.push_str("if (-not $ff) { $ff = (Get-ChildItem \"$env:USERPROFILE\\tools\\ffmpeg-*\\bin\\ffmpeg.exe\" | Select-Object -First 1).FullName }\n");
    s.push_str("if (-not $ff -or -not (Test-Path $ff)) { throw \"ffmpeg not found: put it on PATH or set FFMPEG_DIR\" }\n");
    s.push_str("$fprobe = Join-Path (Split-Path -Parent $ff) 'ffprobe.exe'\n");
    s.push_str("Write-Host \"ffmpeg: $ff\"\n");
    s.push_str("$audio = @(); $video = @()\n");
    for (i, beat) in beats.iter().enumerate() {
        let _ = beat;
        s.push_str(&format!(
            "if (Test-Path 'narration/{i:03}.wav') {{ $a = 'narration/{i:03}.wav' }} else {{\n  & $ff -y -v error -f lavfi -i anullsrc=r=24000:cl=mono -t {SILENT_HOLD} -c:a pcm_s16le 'silence_{i:03}.wav'\n  $a = 'silence_{i:03}.wav' }}\n"
        ));
        s.push_str("$d = [double](& $fprobe -v error -show_entries format=duration -of csv=p=0 $a)\n");
        s.push_str("$audio += \"file '$a'\"\n");
        s.push_str(&format!("$video += \"file 'slides/slide_{i:03}.png'\"\n$video += \"duration $d\"\n"));
    }
    s.push_str(&format!(
        "$video += \"file 'slides/slide_{:03}.png'\"\n",
        beats.len() - 1
    ));
    s.push_str("$audio | Set-Content -Encoding ascii audio_list.txt\n");
    s.push_str("$video | Set-Content -Encoding ascii video_list.txt\n");
    s.push_str("& $ff -y -v error -f concat -safe 0 -i audio_list.txt -c:a pcm_s16le audio.wav\n");
    s.push_str("# h264 needs even dimensions; normalise to 1080p whatever the capture size was.\n");
    s.push_str("& $ff -y -v error -f concat -safe 0 -i video_list.txt -vf \"scale=1920:1080:flags=lanczos,format=yuv420p\" -r 30 -c:v libx264 -preset medium -crf 20 video.mp4\n");
    s.push_str(&format!(
        "& $ff -y -v error -i video.mp4 -i audio.wav -c:v copy -c:a aac -b:a 128k -shortest section_{section:02}.mp4\n"
    ));
    s.push_str(&format!(
        "& $fprobe -v error -show_entries format=duration,size:stream=codec_name,width,height -of default=noprint_wrappers=1 section_{section:02}.mp4\n"
    ));
    s
}
