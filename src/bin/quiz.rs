//! `quiz` — retrieval questions generated from the lessons' own sentences.
//!
//! Retrieval practice beats re-reading, but only if the questions target what the lesson
//! actually asserts. So the questions are not written by hand: each is a sentence from the
//! section whose *answer* is a code span the lesson already marks as important. The span
//! becomes a blank, and the answers sit behind a disclosure so a learner can score
//! themselves without an instructor.
//!
//! Two rules keep a question honest, and both came from a defect the tests caught:
//! a term inside backticks is never rewritten (`dns_query` was being offered as `dnsquery`,
//! because emphasis stripping removed every underscore), and a sentence whose answer is
//! visible anywhere else in the prompt is not asked at all (section 10 opened with
//! "FizzBuzz, … say `FizzBuzz`", so its blank gave itself away).
//!
//! Every section must yield at least [`MIN_QUESTIONS`] items, because a section that cannot
//! be scored must not ship; `cargo test` asserts it and the binary exits non-zero when a
//! section is under the floor.
//!
//! Usage: `cargo run --bin quiz -- [section]` → `problems/section_NN_quiz.md`

use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

use rust_lessons::lesson::{self, Beat, Kind};

/// How many questions a section gets at most.
const MAX_QUESTIONS: usize = 5;

/// How many questions a section must get before it can be scored at all.
///
/// Four is the floor, not a preference: with three or fewer a single wrong answer moves a
/// section's score by a third, and sections that thin were the reason phase 4 measurement
/// had nothing to measure. Section 14 yielded two until it was given prose worth asking
/// about — the fix was more teaching, not a looser generator. `cargo test` asserts this
/// per section, so prose edited down again is a red test rather than a silent gap.
const MIN_QUESTIONS: usize = 4;

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

    let out_dir = PathBuf::from("problems");
    fs::create_dir_all(&out_dir).expect("create problems/");

    let mut thin = Vec::new();
    for n in sections {
        let Some(beats) = lesson::beats(n) else {
            continue;
        };
        let questions = questions_for(&beats);
        if questions.len() < MIN_QUESTIONS {
            thin.push((n, questions.len()));
        }
        let path = out_dir.join(format!("section_{n:02}_quiz.md"));
        fs::write(&path, render(n, &questions)).expect("write quiz");
        println!("section {n:02}: {} question(s) -> {}", questions.len(), path.display());
    }
    if !thin.is_empty() {
        eprintln!("sections below the {MIN_QUESTIONS}-question floor: {thin:?}");
        return std::process::ExitCode::from(1);
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
///
/// A `# Worked example` block is skipped: it is a sequence of steps to follow, not a
/// definition to recall, and letting it count against [`MAX_QUESTIONS`] pushed sections
/// 07, 08, 11 and 16's own definitions — the SQL each operation corresponds to, the
/// `panic!` that needs the never type — out of their quizzes entirely. The block ends at
/// the next `#` heading, so a section that adds an example must also head what follows it;
/// a `cargo run --bin quiz` diff shows the day one does not.
pub fn questions_for(beats: &[Beat]) -> Vec<Question> {
    let mut seen: HashSet<String> = HashSet::new();
    let mut out = Vec::new();
    let mut in_example = false;

    for beat in beats {
        if beat.kind == Kind::Heading {
            in_example = beat.text.trim_start().starts_with("# Worked example");
        }
        if in_example || !matches!(beat.kind, Kind::Prose | Kind::Bullets) {
            continue;
        }
        for line in sentences(&beat.text) {
            if out.len() >= MAX_QUESTIONS {
                return out;
            }
            let Some(answer) = best_code_span(&line) else {
                continue;
            };
            if answer.starts_with('"') || seen.contains(&answer) {
                continue;
            }
            let needle = format!("`{answer}`");
            if !line.contains(&needle) {
                continue;
            }
            let prompt = line.replace(&needle, "**____**");
            // A blank is a question only if the term is nowhere else in the sentence.
            // Section 10 opened "FizzBuzz, the exercise every course uses: … `FizzBuzz`",
            // so the prompt handed back the answer it was asking for. The answer is marked
            // seen only when a question is actually produced, so a later sentence can
            // still ask it cleanly.
            if contains_word(&prompt, &answer) {
                continue;
            }
            seen.insert(answer.clone());
            out.push(Question { prompt, answer });
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
    let flat = strip_emphasis(
        &text
            .lines()
            .map(str::trim)
            .collect::<Vec<_>>()
            .join(" "),
    );
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

/// Drop emphasis markers — but never a character inside a code span.
///
/// A blanket `replace(['*', '_'], "")` renamed `dns_query` to `dnsquery`, so the answer the
/// learner was shown was not the name the lesson uses; an underscore is markdown emphasis
/// only outside backticks. Asterisks are skipped outside code spans too, which is what
/// removes `**bold**` in one pass.
fn strip_emphasis(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut in_code = false;
    for c in text.chars() {
        if c == '`' {
            in_code = !in_code;
            out.push(c);
        } else if in_code || (c != '*' && c != '_') {
            out.push(c);
        }
    }
    out
}

/// Does `hay` contain `needle` as a whole word — not as part of a longer identifier?
///
/// The substring test this replaces reported a leak when the answer was `let` and the
/// prompt said "letters", which is exactly the noise that gets an assertion deleted.
fn contains_word(hay: &str, needle: &str) -> bool {
    let word = |c: char| c.is_alphanumeric() || c == '_';
    hay.match_indices(needle).any(|(start, _)| {
        let before = hay[..start].chars().next_back();
        let after = hay[start + needle.len()..].chars().next();
        let opens = needle.chars().next().is_none_or(|c| !word(c))
            || before.is_none_or(|c| !word(c));
        let closes = needle.chars().next_back().is_none_or(|c| !word(c))
            || after.is_none_or(|c| !word(c));
        opens && closes
    })
}

/// The longest backticked span in a line — the term most worth asking about.
fn best_code_span(line: &str) -> Option<String> {
    let mut best: Option<String> = None;
    let mut parts = line.split('`');
    while parts.next().is_some() {
        let Some(span_text) = parts.next() else { break };
        let candidate = span_text.trim();
        if candidate.len() >= 3 && best.as_ref().is_none_or(|b| candidate.len() > b.len()) {
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
    fn an_underscore_inside_a_code_span_survives_emphasis_stripping() {
        let beats = prose(
            "The `dns_query` helper answers which record type will be asked for, and the caller learns it.",
        );
        let qs = questions_for(&beats);
        assert_eq!(qs.len(), 1);
        assert_eq!(qs[0].answer, "dns_query", "the name the lesson uses, not dnsquery");
    }

    #[test]
    fn short_fragments_and_duplicates_are_skipped() {
        let beats = prose("The `borrow checker` decides when a value is released.\nThe `borrow checker` appears again in every later lesson of this course.");
        let qs = questions_for(&beats);
        assert_eq!(qs.len(), 1, "one distinctive answer asked once, not twice");
        assert!(qs[0].prompt.split_whitespace().count() >= 8, "no fragments as prompts");
    }

    /// Every section must yield at least `MIN_QUESTIONS` scorable items.
    ///
    /// This is the assertion phase 4 depends on: a section below the floor cannot be
    /// scored, so it must not ship. Section 14 yielded two until its prose gained
    /// sentences worth asking about; the guard is here so that editing prose back down is
    /// a red test. Proven non-vacuous on 2026-11-07 by deleting section 05's truthiness
    /// sentence: the run failed with "section 05 yields only 3 retrieval items, below the
    /// floor of 4", and the sentence was restored.
    #[test]
    fn every_section_has_enough_retrieval_items_to_be_scored() {
        for n in 1..=rust_lessons::SECTION_COUNT {
            let beats = lesson::beats(n).expect("beats");
            let qs = questions_for(&beats);
            assert!(
                qs.len() <= MAX_QUESTIONS,
                "section {n:02} yields {} items, over the cap of {MAX_QUESTIONS}",
                qs.len()
            );
            assert!(
                qs.len() >= MIN_QUESTIONS,
                "section {n:02} yields only {} retrieval items, below the floor of {MIN_QUESTIONS}",
                qs.len()
            );
            let mut answers = std::collections::HashSet::new();
            for q in &qs {
                assert!(!q.answer.is_empty(), "section {n:02} has an empty answer");
                assert!(
                    q.prompt.contains("____"),
                    "section {n:02} has a prompt with no blank: {}",
                    q.prompt
                );
                assert!(
                    !contains_word(&q.prompt, &q.answer),
                    "section {n:02} leaks its answer in the prompt: {}",
                    q.prompt
                );
                assert!(
                    answers.insert(q.answer.clone()),
                    "section {n:02} asks about `{}` twice",
                    q.answer
                );
            }
        }
    }
}
