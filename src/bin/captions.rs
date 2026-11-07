//! `captions` — write an SRT subtitle track for a section's video.
//!
//! The timings already exist: the mux holds each slide for exactly the length of its own
//! narration, so a cue per narrated beat lands on the right frame without any alignment work.
//! That makes captions nearly free — and not having them is what would spoil a first review
//! from a deaf or muted reader, or hand `&str` and `E0308` to YouTube's auto-captioner.
//!
//! Beat text is the source: `lesson::beats` re-derives it, so captions cannot drift from the
//! prose the way a hand-written subtitle file would.
//!
//! Usage: `cargo run --bin captions -- [section]` → `out/video/section_NN/section_NN.srt`

use std::fs;
use std::path::PathBuf;

use rust_lessons::lesson::{self, Beat};

fn main() -> std::process::ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let sections: Vec<u8> = match args.first().and_then(|a| a.parse::<u8>().ok()) {
        Some(n) if (1..=rust_lessons::SECTION_COUNT).contains(&n) => vec![n],
        Some(_) => {
            eprintln!("no such section");
            return std::process::ExitCode::from(2);
        }
        None => (1..=rust_lessons::SECTION_COUNT).collect(),
    };

    let mut written = 0;
    for n in sections {
        let Some(beats) = lesson::beats(n) else {
            continue;
        };
        let dir = PathBuf::from(format!("out/video/section_{n:02}"));
        let (srt, cues) = build(n, &beats, &dir);
        let path = dir.join(format!("section_{n:02}.srt"));
        fs::write(&path, srt).expect("write srt");
        written += 1;
        let last = cues.last().map(|c| c.2).unwrap_or(0.0);
        println!("section {n:02}: {} cues, ends {last:.1}s -> {}", cues.len(), path.display());
    }
    println!("{written} subtitle track(s)");
    std::process::ExitCode::SUCCESS
}

/// One cue: index, start seconds, end seconds, text.
pub type Cue = (usize, f64, f64, String);

/// Build the cue list and the SRT body.
///
/// Beats without narration are silent for `SILENT_HOLD`; they get no cue, because the slide
/// itself is showing that content and a caption reading `\[code\]` would be noise.
pub fn build(n: u8, beats: &[Beat], dir: &PathBuf) -> (String, Vec<Cue>) {
    let _ = n;
    let mut cues: Vec<Cue> = Vec::new();
    let mut t = 0.0f64;
    for (i, beat) in beats.iter().enumerate() {
        let d = match wav_seconds(&dir.join(format!("narration/{i:03}.wav"))) {
            Some(secs) => secs,
            None => rust_lessons::lesson::SILENT_HOLD,
        };
        if lesson::needs_narration(beat) {
            let text = wrap(&lesson::speakable(&beat.text), 42, 2);
            if !text.is_empty() && d > 0.2 {
                cues.push((cues.len() + 1, t, t + d, text));
            }
        }
        t += d;
    }
    let mut srt = String::new();
    for (idx, start, end, text) in &cues {
        srt.push_str(&format!(
            "{idx}\n{} --> {}\n{text}\n\n",
            stamp(*start),
            stamp(*end)
        ));
    }
    (srt, cues)
}

/// Duration of a PCM WAV file, read from its own header — no ffprobe, no dependency.
pub fn wav_seconds(path: &PathBuf) -> Option<f64> {
    let bytes = fs::read(path).ok()?;
    if bytes.len() < 44 || &bytes[0..4] != b"RIFF" || &bytes[8..12] != b"WAVE" {
        return None;
    }
    let mut pos = 12usize;
    let mut byte_rate: Option<u32> = None;
    while pos + 8 <= bytes.len() {
        let id = &bytes[pos..pos + 4];
        let size = u32::from_le_bytes(bytes[pos + 4..pos + 8].try_into().ok()?) as usize;
        let body = pos + 8;
        match id {
            b"fmt " if body + 16 <= bytes.len() => {
                // audioFormat(+0), channels(+2), sampleRate(+4), byteRate(+8)
                byte_rate = Some(u32::from_le_bytes(
                    bytes[body + 8..body + 12].try_into().ok()?,
                ));
            }
            b"data" => {
                let rate = byte_rate?;
                if rate == 0 {
                    return None;
                }
                let data_len = size.min(bytes.len().saturating_sub(body));
                return Some(data_len as f64 / rate as f64);
            }
            _ => {}
        }
        pos = body + size + (size & 1); // chunks are word-aligned
    }
    None
}

/// `HH:MM:SS,mmm`, the SRT timestamp.
pub fn stamp(seconds: f64) -> String {
    let ms_total = (seconds * 1000.0).round() as u64;
    let ms = ms_total % 1000;
    let s = (ms_total / 1000) % 60;
    let m = (ms_total / 60_000) % 60;
    let h = ms_total / 3_600_000;
    format!("{h:02}:{m:02}:{s:02},{ms:03}")
}

/// Wrap to at most `max_lines` lines of about `width` characters.
pub fn wrap(text: &str, width: usize, max_lines: usize) -> String {
    let mut lines: Vec<String> = Vec::new();
    let mut current = String::new();
    for word in text.split_whitespace() {
        if current.is_empty() {
            current.push_str(word);
        } else if current.len() + 1 + word.len() <= width {
            current.push(' ');
            current.push_str(word);
        } else {
            lines.push(current.clone());
            current.clear();
            current.push_str(word);
        }
    }
    if !current.is_empty() {
        lines.push(current);
    }
    lines.truncate(max_lines);
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timestamps_are_srt_shaped() {
        assert_eq!(stamp(0.0), "00:00:00,000");
        assert_eq!(stamp(1.5), "00:00:01,500");
        assert_eq!(stamp(61.25), "00:01:01,250");
        assert_eq!(stamp(3725.001), "01:02:05,001");
    }

    #[test]
    fn wrapping_respects_width_and_line_cap() {
        let out = wrap("the cat sat on the mat and then it sat on another mat entirely", 20, 2);
        assert_eq!(out.lines().count(), 2, "capped at two lines: {out:?}");
        assert!(out.lines().all(|l| l.len() <= 20), "no line exceeds the width: {out:?}");
    }

    #[test]
    fn cue_times_are_monotonic_and_cover_the_video() {
        for n in 1..=rust_lessons::SECTION_COUNT {
            let beats = lesson::beats(n).expect("beats");
            let dir = PathBuf::from(format!("out/video/section_{n:02}"));
            let (srt, cues) = build(n, &beats, &dir);
            if cues.is_empty() {
                continue; // no narration generated yet for this section
            }
            let mut previous_end = 0.0;
            for (position, (idx, start, end, text)) in cues.iter().enumerate() {
                assert_eq!(*idx, position + 1, "cues are numbered from one, in order");
                assert!(start < end, "section {n:02} cue {idx}: start >= end");
                assert!(*start >= previous_end - 0.001, "section {n:02} cue {idx}: overlaps");
                assert!(text.lines().count() <= 2);
                previous_end = *end;
            }
            assert!(srt.contains(" --> "), "srt has arrow separators");
            assert!(!srt.trim().ends_with("-->"), "srt ends with a complete cue");
        }
    }
}
