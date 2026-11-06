//! Turning a lesson module into beats — the shared step between the docs, the slides
//! and the video.
//!
//! A **beat** is one thing on screen: a paragraph, a code block, a table, a bullet
//! list. The lesson text is the only source. `narrate` renders beats as slides, but the
//! same parse is what `tests/curriculum.rs` checks, so a section that loses its prose
//! fails the build rather than producing a silent video.

use std::fs;
use std::path::{Path, PathBuf};

/// What a beat is, which decides both its rendering and whether it is spoken.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// A `#` heading from the prose.
    Heading,
    /// Ordinary prose: spoken.
    Prose,
    /// A fenced code block: shown, not spoken.
    Code,
    /// A `-`/`*` list: spoken.
    Bullets,
    /// A markdown table: shown, not spoken.
    Table,
    /// A pointer to a diagram: shown, not spoken.
    Diagram,
}

/// One slide's worth of lesson.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Beat {
    /// What kind of thing this is.
    pub kind: Kind,
    /// The raw markdown, fences and all.
    pub text: String,
}

/// Should this beat be read aloud?
pub fn needs_narration(beat: &Beat) -> bool {
    matches!(beat.kind, Kind::Prose | Kind::Bullets | Kind::Heading)
}

/// The lesson module for section `n` (`src/section_NN_*.rs`).
pub fn module_path(n: u8) -> Option<PathBuf> {
    let prefix = format!("section_{n:02}_");
    fs::read_dir(Path::new("src"))
        .ok()?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .find(|p| {
            p.file_name()
                .and_then(|f| f.to_str())
                .map(|f| f.starts_with(&prefix) && f.ends_with(".rs"))
                .unwrap_or(false)
        })
}

/// The beats of section `n`, or `None` when the module is missing.
pub fn beats(n: u8) -> Option<Vec<Beat>> {
    let path = module_path(n)?;
    let source = fs::read_to_string(path).ok()?;
    let prose = doc_block(&source);
    let beats = split_beats(&prose);
    if beats.is_empty() {
        None
    } else {
        Some(beats)
    }
}

/// The lesson as written: every `//!` header line *and* every `///` item doc, in file
/// order, up to the test module.
///
/// Reading both matters. Section 1's opening lives in the header, but its three
/// teaching paragraphs are `///` docs on the items they explain — extracting only the
/// header produced four beats for a lesson with ten.
pub fn doc_block(source: &str) -> String {
    let mut out = String::new();
    for line in source.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("#[cfg(test)]") {
            break;
        }
        let text = trimmed
            .strip_prefix("//!")
            .or_else(|| trimmed.strip_prefix("///"))
            .map(|rest| rest.strip_prefix(' ').unwrap_or(rest));
        match text {
            Some(t) => {
                out.push_str(t);
                out.push('\n');
            }
            None => out.push('\n'),
        }
    }
    out
}

/// Split prose into beats: blank-line separated blocks, with each fenced code block
/// kept whole.
pub fn split_beats(prose: &str) -> Vec<Beat> {
    let mut beats = Vec::new();
    let mut block: Vec<&str> = Vec::new();
    let mut in_fence = false;

    fn push(beats: &mut Vec<Beat>, lines: &[&str]) {
        let text = lines.join("\n").trim_end().to_string();
        if text.is_empty() {
            return;
        }
        let first = text.lines().next().unwrap_or("").trim_start();
        let kind = if first.starts_with('#') {
            Kind::Heading
        } else if first.starts_with("```") || first.starts_with("    ") {
            Kind::Code
        } else if first.starts_with("| ") {
            Kind::Table
        } else if first.starts_with("- ") || first.starts_with("* ") {
            Kind::Bullets
        } else if first.contains("docs/diagrams/") {
            Kind::Diagram
        } else {
            Kind::Prose
        };
        beats.push(Beat { kind, text });
    }

    for line in prose.lines() {
        let t = line.trim_start();
        if t.starts_with("```") {
            if in_fence {
                block.push(line);
                push(&mut beats, &block);
                block.clear();
                in_fence = false;
            } else {
                if !block.is_empty() {
                    push(&mut beats, &block);
                    block.clear();
                }
                in_fence = true;
                block.push(line);
            }
            continue;
        }
        if in_fence {
            block.push(line);
            continue;
        }
        if line.trim().is_empty() {
            if !block.is_empty() {
                push(&mut beats, &block);
                block.clear();
            }
            continue;
        }
        block.push(line);
    }
    if !block.is_empty() {
        push(&mut beats, &block);
    }
    beats
}

/// Strip markdown the speaker should not read: backticks, emphasis, list markers.
pub fn speakable(text: &str) -> String {
    let mut out = String::new();
    for line in text.lines() {
        let line = line.trim_start_matches(['-', '*', '#', ' ']);
        let cleaned: String = line
            .chars()
            .filter(|c| *c != '`' && *c != '*')
            .collect::<String>()
            .replace("  ", " ");
        if !cleaned.trim().is_empty() {
            out.push_str(cleaned.trim());
            out.push(' ');
        }
    }
    out.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn both_doc_forms_are_read() {
        let src = "//! a header line\n//!\n/// an item line\npub fn x() {}\n#[cfg(test)]\nmod tests { /// not lesson }\n";
        let prose = doc_block(src);
        assert!(prose.contains("a header line"));
        assert!(prose.contains("an item line"));
        assert!(!prose.contains("not lesson"), "tests are not lesson text");
    }

    #[test]
    fn fences_stay_whole_and_kinds_are_classified() {
        let prose = "First paragraph.\n\n```rust\nfn a() {}\n\nfn b() {}\n```\n\n- one\n- two\n\n| a | b |\n";
        let beats = split_beats(prose);
        let kinds: Vec<Kind> = beats.iter().map(|b| b.kind).collect();
        assert_eq!(kinds, vec![Kind::Prose, Kind::Code, Kind::Bullets, Kind::Table]);
        assert!(beats[1].text.contains("fn b()"), "the blank line inside the fence survived");
        assert!(needs_narration(&beats[0]));
        assert!(!needs_narration(&beats[1]), "code is shown, not spoken");
    }

    #[test]
    fn speakable_drops_markdown() {
        assert_eq!(speakable("**bold** and `code`"), "bold and code");
        assert_eq!(speakable("- a bullet"), "a bullet");
    }
}
