//! Scoring a recorded session: correctness, time to answer, retrieval versus recognition.
//!
//! This is roadmap phase 4's completion criterion — "Parsons + retrieval artefacts are
//! scoreable; a section ends with 'you can now do X' and a task that produces a number.
//! *Done when* a stranger's score on section 01 can be recorded without an instructor
//! present" — made concrete as arithmetic over [`Session`]. The input is
//! `fixtures/session-2026-11-07-synthetic.json`; the output is [`Score`], or the table
//! [`Score::report`] prints.
//!
//! # Three axes per section, one figure overall
//!
//! * **Correctness** — correct tries over tries, derived from `answer` versus `expected`
//!   by [`Attempt::is_correct`], never stored. A wrong-then-right pair therefore scores
//!   one wrong try and one right try, which is what happened.
//! * **Time to answer** — mean milliseconds per try, and the total, both from the recorded
//!   `ms`. A retry's seconds count: the time was spent.
//! * **Retrieval versus recognition** — tallied separately, because a learner who can
//!   recognise working code but not recall a name has a different problem from one who can
//!   do neither, and averaging the two hides exactly that.
//!
//! The overall figure uses the same structure as a section, so there is one formula and no
//! second, disagreeing path for the total.
//!
//! # Tries, not items
//!
//! An item can be tried more than once, so a section prints `items` and `tries` as two
//! columns: a retry is visible in the table rather than folded into a denominator. This
//! matters for the fixture, where section 01 has three items and four tries.
//!
//! # A mode with no items prints `-`, not `0.0%`
//!
//! Zero of zero is not zero percent: it means the section did not ask that kind of item
//! (section 02 of the fixture asks no Parsons problem). [`Tally::accuracy`] returns
//! [`None`] there, and the report says so, because a silent zero would read as a failure
//! the learner never had the chance to commit.
//!
//! # What these numbers are not
//!
//! A score describes a session, not a person: the checked-in fixture is synthetic, so its
//! table is evidence about the reader and the arithmetic and about nobody's learning. n = 1
//! is not a study, and [`Score::synthetic`] travels with the numbers so a copied table
//! cannot quietly pose as a human result.

use std::collections::{BTreeMap, BTreeSet};

use crate::session::{Attempt, Mode, Session};

/// Answers given, and answers right, for a section, a mode, or a whole session.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Tally {
    /// How many answers were given.
    pub tries: usize,
    /// How many of them matched the item's expected answer.
    pub correct: usize,
}

impl Tally {
    fn add(&mut self, attempt: &Attempt) {
        self.tries += 1;
        if attempt.is_correct() {
            self.correct += 1;
        }
    }

    /// Correct tries over tries, or [`None`] when nothing of that kind was asked.
    ///
    /// `None` rather than `0.0`, so an untested axis cannot be reported as a failure.
    pub fn accuracy(&self) -> Option<f64> {
        if self.tries == 0 {
            None
        } else {
            Some(self.correct as f64 / self.tries as f64)
        }
    }

    /// `2/3 (66.7%)`, or `-` when nothing of that kind was asked.
    pub fn label(&self) -> String {
        match self.accuracy() {
            Some(accuracy) => format!("{}/{} ({:.1}%)", self.correct, self.tries, accuracy * 100.0),
            None => "-".into(),
        }
    }
}

/// One section's score: one row of the table. The overall figure is the same type with
/// `section == "overall"`, so both are computed by the same code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SectionScore {
    /// Two-digit section number, or `overall`.
    pub section: String,
    /// Distinct items asked, so a retry does not look like a new item.
    pub items: usize,
    /// Every try, whatever the mode.
    pub attempts: Tally,
    /// Tries at items answered from memory.
    pub retrieval: Tally,
    /// Tries at items answered with the material visible.
    pub recognition: Tally,
    /// Total time on task, milliseconds.
    pub total_ms: u64,
}

impl SectionScore {
    fn new(section: &str) -> Self {
        SectionScore {
            section: section.to_string(),
            items: 0,
            attempts: Tally::default(),
            retrieval: Tally::default(),
            recognition: Tally::default(),
            total_ms: 0,
        }
    }

    fn add(&mut self, attempt: &Attempt) {
        self.attempts.add(attempt);
        match attempt.mode {
            Mode::Retrieval => self.retrieval.add(attempt),
            Mode::Recognition => self.recognition.add(attempt),
        }
        self.total_ms += attempt.ms;
    }

    /// Correct tries over tries for the section.
    pub fn accuracy(&self) -> Option<f64> {
        self.attempts.accuracy()
    }

    /// Mean milliseconds per try. The session loader refuses a zero duration, so this is
    /// a real mean and not a number that averaged a missing timing in.
    pub fn mean_ms(&self) -> f64 {
        self.total_ms as f64 / self.attempts.tries as f64
    }
}

/// A whole session's score: one row per section touched, plus the overall figure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Score {
    /// The session this score describes.
    pub session_id: String,
    /// Who sat it; `synthetic-fixture` means no person did.
    pub learner: String,
    /// True when no human produced the answers.
    pub synthetic: bool,
    /// One row per section, in section order.
    pub sections: Vec<SectionScore>,
    /// The same three axes over the whole session.
    pub overall: SectionScore,
}

/// Score every attempt in a session.
///
/// Sections come out sorted (`BTreeMap`, not a hash map), so the same session always prints
/// the same table in the same order.
pub fn score(session: &Session) -> Score {
    type Row<'a> = (BTreeSet<&'a str>, SectionScore);
    let mut rows: BTreeMap<&str, Row> = BTreeMap::new();
    let mut overall = SectionScore::new("overall");
    let mut overall_items: BTreeSet<(&str, &str)> = BTreeSet::new();

    for attempt in &session.items {
        let row = rows
            .entry(attempt.section.as_str())
            .or_insert_with(|| (BTreeSet::new(), SectionScore::new(&attempt.section)));
        row.0.insert(attempt.prompt.as_str());
        row.1.add(attempt);

        overall_items.insert(attempt.item());
        overall.add(attempt);
    }

    let sections: Vec<SectionScore> = rows
        .into_values()
        .map(|(items, mut row)| {
            row.items = items.len();
            row
        })
        .collect();
    overall.items = overall_items.len();

    Score {
        session_id: session.session_id.clone(),
        learner: session.learner.clone(),
        synthetic: session.synthetic,
        sections,
        overall,
    }
}

/// The column widths the table uses; header and rows share them, so they cannot drift.
const LABEL: usize = 8;
const ITEMS: usize = 6;
const TRIES: usize = 7;
const CORRECT: usize = 9;
const ACCURACY: usize = 10;
const MEAN: usize = 9;
const TOTAL: usize = 9;
const MODE: usize = 18;

fn header() -> String {
    format!(
        "{:<LABEL$}{:>ITEMS$}{:>TRIES$}{:>CORRECT$}{:>ACCURACY$}{:>MEAN$}{:>TOTAL$}  {:<MODE$}{}",
        "section", "items", "tries", "correct", "accuracy", "mean s", "total s", "retrieval",
        "recognition"
    )
    .trim_end()
    .to_string()
}

fn row(score: &SectionScore) -> String {
    let accuracy = match score.accuracy() {
        Some(accuracy) => format!("{:.1}%", accuracy * 100.0),
        None => "-".into(),
    };
    format!(
        "{:<LABEL$}{:>ITEMS$}{:>TRIES$}{:>CORRECT$}{:>ACCURACY$}{:>MEAN$}{:>TOTAL$}  {:<MODE$}{}",
        score.section,
        score.items,
        score.attempts.tries,
        score.attempts.correct,
        accuracy,
        format!("{:.1}", score.mean_ms() / 1000.0),
        format!("{:.1}", score.total_ms as f64 / 1000.0),
        score.retrieval.label(),
        score.recognition.label()
    )
    .trim_end()
    .to_string()
}

impl Score {
    /// The table, as text. One row per section on the three axes, then the overall figure.
    pub fn report(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!(
            "session {} | learner: {} | {}\n\n",
            self.session_id,
            self.learner,
            if self.synthetic {
                "synthetic - written, not sat"
            } else {
                "recorded from a person"
            }
        ));
        out.push_str(&header());
        out.push('\n');
        for section in &self.sections {
            out.push_str(&row(section));
            out.push('\n');
        }
        out.push_str(&row(&self.overall));
        out.push('\n');
        out.push_str("\nlast row is the whole session; '-' means no item of that kind was asked.\n");
        if self.synthetic {
            out.push_str(
                "synthetic session: these numbers describe the reader and the arithmetic, \
                 never a person's learning.\n",
            );
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::session::SCHEMA;

    fn attempt(
        section: &str,
        prompt: &str,
        mode: Mode,
        attempt: u8,
        answer: &str,
        expected: &str,
        ms: u64,
    ) -> Attempt {
        Attempt {
            section: section.into(),
            prompt: prompt.into(),
            mode,
            attempt,
            answer: answer.into(),
            expected: expected.into(),
            ms,
        }
    }

    fn session(items: Vec<Attempt>) -> Session {
        Session {
            schema: SCHEMA,
            session_id: "t".into(),
            learner: "synthetic-fixture".into(),
            synthetic: true,
            started: "2026-11-07T08:40:00Z".into(),
            items,
        }
    }

    #[test]
    fn retrieval_and_recognition_are_scored_apart() {
        let session = session(vec![
            attempt("01", "s01-q1", Mode::Retrieval, 1, "let", "let", 1000),
            attempt("01", "s01-q2", Mode::Retrieval, 1, "var", "let", 3000),
            attempt("01", "s01-parsons", Mode::Recognition, 1, "2,1", "1,2", 5000),
        ]);
        let section = &score(&session).sections[0];
        assert_eq!(section.retrieval, Tally { tries: 2, correct: 1 });
        assert_eq!(section.recognition, Tally { tries: 1, correct: 0 });
        assert_eq!(section.attempts, Tally { tries: 3, correct: 1 });
        assert_eq!(section.items, 3);
    }

    #[test]
    fn a_retry_is_two_tries_of_one_item_and_both_times_count() {
        let session = session(vec![
            attempt("01", "s01-q2", Mode::Retrieval, 1, "var", "let", 6000),
            attempt("01", "s01-q2", Mode::Retrieval, 2, "let", "let", 2000),
        ]);
        let section = &score(&session).sections[0];
        assert_eq!(section.items, 1, "one item, retried");
        assert_eq!(section.attempts.tries, 2, "two tries at it");
        assert_eq!(section.attempts.correct, 1);
        assert_eq!(section.total_ms, 8000);
        assert_eq!(section.mean_ms(), 4000.0, "mean per try, not per item");
    }

    #[test]
    fn an_untested_mode_is_unknown_not_zero_percent() {
        let session = session(vec![attempt(
            "02",
            "s02-q1",
            Mode::Retrieval,
            1,
            "let",
            "let",
            1000,
        )]);
        let section = &score(&session).sections[0];
        assert_eq!(section.recognition.accuracy(), None);
        assert_eq!(section.recognition.label(), "-");
        assert_eq!(section.accuracy(), Some(1.0));
    }

    #[test]
    fn the_overall_figure_uses_the_same_formula_as_a_section() {
        let session = session(vec![
            attempt("01", "s01-q1", Mode::Retrieval, 1, "let", "let", 1000),
            attempt("02", "s02-q1", Mode::Retrieval, 1, "var", "let", 3000),
        ]);
        let score = score(&session);
        assert_eq!(score.sections.len(), 2);
        assert_eq!(score.overall.section, "overall");
        assert_eq!(score.overall.items, 2);
        assert_eq!(score.overall.attempts, Tally { tries: 2, correct: 1 });
        assert_eq!(score.overall.total_ms, 4000);
        assert_eq!(score.overall.accuracy(), Some(0.5));
    }

    #[test]
    fn a_section_that_contradicts_its_own_text_is_scored_by_its_text() {
        // Same item, same recorded time; only the answer differs. The score follows the
        // answer, because no verdict was ever stored in the data.
        let right = score(&session(vec![attempt(
            "01",
            "s01-q1",
            Mode::Retrieval,
            1,
            "`let`",
            "let",
            1000,
        )]));
        let wrong = score(&session(vec![attempt(
            "01",
            "s01-q1",
            Mode::Retrieval,
            1,
            "let mut",
            "let",
            1000,
        )]));
        assert_eq!(right.overall.attempts.correct, 1);
        assert_eq!(wrong.overall.attempts.correct, 0);
    }

    #[test]
    fn sections_are_reported_in_section_order() {
        let session = session(vec![
            attempt("03", "s03-q1", Mode::Retrieval, 1, "a", "a", 1000),
            attempt("01", "s01-q1", Mode::Retrieval, 1, "b", "b", 1000),
            attempt("02", "s02-q1", Mode::Retrieval, 1, "c", "c", 1000),
        ]);
        let sections: Vec<String> = score(&session)
            .sections
            .iter()
            .map(|s| s.section.clone())
            .collect();
        assert_eq!(sections, ["01", "02", "03"]);
    }
}
