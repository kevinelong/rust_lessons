//! `quiz` — retrieval questions generated from the lessons' own sentences.
//!
//! Retrieval practice beats re-reading, but only if the questions target what the lesson
//! actually asserts. So the questions are not written by hand: each is a sentence from the
//! section whose *answer* is a code span the lesson already marks as important. The span
//! becomes a blank, and the answers sit behind a disclosure so a learner can score
//! themselves without an instructor.
//!
//! Usage: `cargo run --bin quiz -- [section]` → `problems/section_NN_quiz.md`

use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

use rust_lessons::lesson::{self, Beat, Kind};

/// How many questions a section gets at most.
const MAX_QUESTIONS: usize = 5;

fn main() -> std::process::ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let sections: Vec<u8> = match args.first().and_then(|a| a.parse::<u8>().ok()) {
        Some(n) if (1..=20).contains(&n) => vec![n],
        Some(_) => {
            eprintln!("no such section");
            return std::process::ExitCode::from(2);
        }
        None => (1..=20).collect(),
    };

    let out_dir = PathBuf::from("problems");
    fs::create_dir_all(&out_dir).expect("create problems/");

    let mut thin = Vec::new();
    for n in sections {
        let Some(beats) = lesson::beats(n) else {
            continue;
        };
        let questions = questions_for(&beats);
        if questions.len() < 3 {
            thin.push((n, questions.len()));
        }
        let path = out_dir.join(format!("section_{n:02}_quiz.md"));
        fs::write(&path, render(n, &questions)).expect("write quiz");
        println!("section {n:02}: {} question(s) -> {}", questions.len(), path.display());
    }
    if !thin.is_empty() {
        println!("sections with fewer than three questions: {thin:?}");
    }
    std::process::ExitCode::SUCCESS
}

/// One cloze question: the sentence with the span blanked, plus the answer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Question {
    pub prompt: String,
    pub answer: String,
}

/// Pull questions out of the prose beats, in document order, deduplicated by answer.
pub fn questions_for(beats: &[Beat]) -> Vec<Question> {
    let mut seen: HashSet<String> = HashSet::new();
    let mut out = Vec::new();

    for beat in beats {
        if !matches!(beat.kind, Kind::Prose | Kind::Bullets) {
            continue;
        }
        for line in sentences(&beat.text) {
            if out.len() >= MAX_QUESTIONS {
                return out;
            }
            let Some(answer) = best_code_span(&line) else {
                continue;
            };
            if answer.starts_with('"') || !seen.insert(answer.clone()) {
                continue;
            }
            let needle = format!("`{answer}`");
            if !line.contains(&needle) {
                continue;
            }
            out.push(Question {
                prompt: line.replace(&needle, "**____**"),
                answer,
            });
        }
    }
    out
}

/// Sentences from a beat, unwrapped first.
///
/// Two wrong versions preceded this one, both worth remembering: splitting the raw text on
/// `.` broke inside `**bold**` and parentheticals, and treating each line as a sentence
/// broke because the lessons are hard-wrapped at ~95 columns. The fix is to join the beat's
/// lines into a paragraph, drop emphasis markers, then split only where a full stop is
/// followed by a capital — and to keep only sentences of at least eight words.
fn sentences(text: &str) -> Vec<String> {
    let flat = text
        .lines()
        .map(str::trim)
        .collect::<Vec<_>>()
        .join(" ")
        .replace("**", "")
        .replace(['*', '_'], "");
    let flat = flat.split_whitespace().collect::<Vec<_>>().join(" ");
    let chars: Vec<char> = flat.chars().collect();

    let mut out = Vec::new();
    let mut current = String::new();
    for (i, c) in chars.iter().enumerate() {
        current.push(*c);
        if *c != '.' {
            continue;
        }
        let ends_here = i + 1 == chars.len()
            || (chars.get(i + 1) == Some(&' ')
                && chars.get(i + 2).map(|n| n.is_uppercase()).unwrap_or(false));
        if ends_here {
            out.push(current.trim().to_string());
            current.clear();
        }
    }
    if !current.trim().is_empty() {
        out.push(current.trim().to_string());
    }
    out.into_iter()
        .filter(|s| s.split_whitespace().count() >= 8)
        .collect()
}

/// The longest backticked span in a line — the term most worth asking about.
fn best_code_span(line: &str) -> Option<String> {
    let mut best: Option<String> = None;
    let mut parts = line.split('`');
    while parts.next().is_some() {
        let Some(span_text) = parts.next() else { break };
        let candidate = span_text.trim();
        if candidate.len() >= 3 && best.as_ref().map_or(true, |b| candidate.len() > b.len()) {
            best = Some(candidate.to_string());
        }
    }
    best
}

fn render(n: u8, questions: &[Question]) -> String {
    let title = rust_lessons::COURSE_MAP
        .iter()
        .chain(rust_lessons::TRACK_B.iter())
        .find(|(num, _)| *num == format!("{n:02}"))
        .map(|(_, t)| t.to_string())
        .unwrap_or_else(|| format!("Section {n}"));

    let mut out = String::new();
    out.push_str(&format!("# Retrieval check — section {n:02}: {title}\n\n"));
    out.push_str("Answer from memory before looking. Each question comes from a sentence in the lesson, with one term removed.\n\n");
    for (i, q) in questions.iter().enumerate() {
        out.push_str(&format!("{}. {}\n\n", i + 1, q.prompt.trim()));
    }
    if questions.is_empty() {
        out.push_str("_This section states no testable sentence in prose; use its Parsons problem instead._\n\n");
    }
    out.push_str("<details>\n<summary>Answers</summary>\n\n");
    for (i, q) in questions.iter().enumerate() {
        out.push_str(&format!("{}. `{}`\n", i + 1, q.answer));
    }
    out.push_str(&format!(
        "\n</details>\n\n<sub>Generated from `src/section_{n:02}_*.rs` by `cargo run --bin quiz` — rust_lessons {}</sub>\n",
        rust_lessons::VERSION
    ));
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn prose(text: &str) -> Vec<Beat> {
        vec![Beat {
            kind: Kind::Prose,
            text: text.to_string(),
        }]
    }

    #[test]
    fn a_code_span_becomes_the_blank_and_the_answer() {
        let beats = prose("The `borrow checker` decides when a value is released.");
        let qs = questions_for(&beats);
        assert_eq!(qs.len(), 1);
        assert!(qs[0].prompt.contains("**____**"), "prompt: {}", qs[0].prompt);
        assert_eq!(qs[0].answer, "borrow checker");
        assert!(!qs[0].prompt.contains("borrow checker"), "the answer is removed");
    }

    #[test]
    fn short_fragments_and_duplicates_are_skipped() {
        let beats = prose("The `borrow checker` decides when a value is released.\nThe `borrow checker` appears again in every later lesson of this course.");
        let qs = questions_for(&beats);
        assert_eq!(qs.len(), 1, "one distinctive answer asked once, not twice");
        assert!(qs[0].prompt.split_whitespace().count() >= 8, "no fragments as prompts");
    }

    #[test]
    fn every_section_offers_a_retrieval_check() {
        let mut with_questions = 0;
        for n in 1..=20u8 {
            let beats = lesson::beats(n).expect("beats");
            let qs = questions_for(&beats);
            assert!(qs.len() <= MAX_QUESTIONS, "section {n} exceeded the cap");
            if !qs.is_empty() {
                with_questions += 1;
                for q in &qs {
                    assert!(!q.answer.is_empty());
                    assert!(q.prompt.contains("____"));
                }
            }
        }
        assert!(
            with_questions >= 15,
            "expected most sections to yield questions, got {with_questions}"
        );
    }
}
