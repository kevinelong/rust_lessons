//! Phase 4's completion criterion against the one recorded session: per-section
//! correctness, time to answer, and retrieval versus recognition, plus the overall figure.
//!
//! Acceptance for `phase4-scoring` in `docs/PLAN.json`: scoring reads the fixture and
//! returns a table with those three axes per section, and a test asserts the numbers for
//! the fixture exactly. This file is that test. It also exercises the printed table, since
//! a claim that a surface prints is a claim about text.
//!
//! The fixture is synthetic, so nothing here is evidence about a person's learning. What it
//! does prove: the arithmetic over the only session this repository records, pinned number
//! by number, and the table a stranger's session would produce.

use rust_lessons::scoring::{score, Score, SectionScore, Tally};
use rust_lessons::session::Session;

const FIXTURE_PATH: &str = "fixtures/session-2026-11-07-synthetic.json";
const FIXTURE: &str = include_str!("../fixtures/session-2026-11-07-synthetic.json");

fn fixture() -> Session {
    Session::from_json_str(FIXTURE).unwrap_or_else(|e| panic!("{FIXTURE_PATH} must load: {e}"))
}

fn scored() -> Score {
    score(&fixture())
}

/// One row of the score: a section's, or the overall figure (which is a row of the same
/// type, so the same assertions apply to both).
fn section(score: &Score, id: &str) -> SectionScore {
    score
        .sections
        .iter()
        .find(|s| s.section == id)
        .unwrap_or_else(|| {
            assert_eq!(id, score.overall.section, "no row called {id} in the score");
            &score.overall
        })
        .clone()
}

/// Every number the fixture scores to, section by section and overall, pinned.
///
/// The totals are the fixture's own: 8 tries, 5 right, 102 400 ms. Retrieval is 4/6 and
/// recognition 1/2, which is the split the third axis exists to show.
#[test]
fn the_fixture_scores_to_these_numbers_exactly() {
    let score = scored();
    assert_eq!(score.session_id, "synthetic-2026-11-07-a");
    assert_eq!(score.learner, "synthetic-fixture");
    assert!(score.synthetic);

    assert_eq!(
        score.sections,
        vec![
            SectionScore {
                section: "01".into(),
                // Three items: s01-q1, s01-q2 (tried twice), s01-parsons.
                items: 3,
                attempts: Tally { tries: 4, correct: 2 },
                retrieval: Tally { tries: 3, correct: 2 },
                // The Parsons answer has two lines transposed, so it is wrong.
                recognition: Tally { tries: 1, correct: 0 },
                total_ms: 53_700,
            },
            SectionScore {
                section: "02".into(),
                items: 2,
                attempts: Tally { tries: 2, correct: 1 },
                retrieval: Tally { tries: 2, correct: 1 },
                // No Parsons problem was sat in section 02.
                recognition: Tally { tries: 0, correct: 0 },
                total_ms: 9_400,
            },
            SectionScore {
                section: "03".into(),
                items: 2,
                attempts: Tally { tries: 2, correct: 2 },
                retrieval: Tally { tries: 1, correct: 1 },
                recognition: Tally { tries: 1, correct: 1 },
                total_ms: 39_300,
            },
        ]
    );

    assert_eq!(
        score.overall,
        SectionScore {
            section: "overall".into(),
            items: 7,
            attempts: Tally { tries: 8, correct: 5 },
            retrieval: Tally { tries: 6, correct: 4 },
            recognition: Tally { tries: 2, correct: 1 },
            total_ms: 102_400,
        }
    );

    // The three axes as fractions, not only as counts: 5/8 overall, 4/6 retrieval,
    // 1/2 recognition.
    assert_eq!(score.overall.accuracy(), Some(5.0 / 8.0));
    assert_eq!(score.overall.retrieval.accuracy(), Some(4.0 / 6.0));
    assert_eq!(score.overall.recognition.accuracy(), Some(1.0 / 2.0));
    assert_eq!(score.overall.mean_ms(), 12_800.0);
}

/// Per-section times, exactly, in the unit a report prints.
#[test]
fn time_to_answer_is_pinned_per_section_and_overall() {
    let score = scored();
    let seconds = |id: &str| section(&score, id).mean_ms() / 1000.0;
    let total = |id: &str| section(&score, id).total_ms as f64 / 1000.0;

    assert_eq!(seconds("01"), 13.425);
    assert_eq!(seconds("02"), 4.7);
    assert_eq!(seconds("03"), 19.65);
    assert_eq!(seconds("overall"), 12.8);

    assert_eq!(total("01"), 53.7);
    assert_eq!(total("02"), 9.4);
    assert_eq!(total("03"), 39.3);
    assert_eq!(total("overall"), 102.4);

    // The mean counts tries, so the retried item's 6.1 s and 2.4 s are both in 01's mean.
    assert_eq!(section(&score, "01").items, 3);
    assert_eq!(section(&score, "01").attempts.tries, 4);
}

/// The table prints one row per section on all three axes, then the overall figure, and
/// each row's numbers are exactly the pinning test's numbers.
#[test]
fn the_report_shows_the_three_axes_per_section_and_an_overall_row() {
    let report = scored().report();
    let lines: Vec<&str> = report.lines().collect();

    let header = lines
        .iter()
        .find(|line| line.starts_with("section"))
        .expect("the table has a header");
    for axis in ["correct", "accuracy", "mean s", "total s", "retrieval", "recognition"] {
        assert!(header.contains(axis), "header lacks the {axis} axis: {header:?}");
    }

    let row = |label: &str| {
        lines
            .iter()
            .find(|line| line.split_whitespace().next() == Some(label))
            .copied()
            .unwrap_or_else(|| panic!("no row for {label} in\n{report}"))
    };

    assert_eq!(
        row("01").split_whitespace().collect::<Vec<_>>(),
        ["01", "3", "4", "2", "50.0%", "13.4", "53.7", "2/3", "(66.7%)", "0/1", "(0.0%)"]
    );
    assert_eq!(
        row("02").split_whitespace().collect::<Vec<_>>(),
        ["02", "2", "2", "1", "50.0%", "4.7", "9.4", "1/2", "(50.0%)", "-"]
    );
    assert_eq!(
        row("03").split_whitespace().collect::<Vec<_>>(),
        ["03", "2", "2", "2", "100.0%", "19.6", "39.3", "1/1", "(100.0%)", "1/1", "(100.0%)"]
    );
    assert_eq!(
        row("overall").split_whitespace().collect::<Vec<_>>(),
        ["overall", "7", "8", "5", "62.5%", "12.8", "102.4", "4/6", "(66.7%)", "1/2", "(50.0%)"]
    );

    // The overall row is the last numeric row, so a reader cannot mistake a section for it.
    let last = lines
        .iter()
        .rev()
        .find(|line| line.split_whitespace().next() == Some("overall"))
        .expect("an overall row");
    assert!(lines.iter().position(|line| line == last) > lines.iter().position(|line| line == &row("03")));
}

/// A section that asked no item of a mode says `-`, not `0.0%`: zero of zero is unknown,
/// and a silent zero would read as a failure the learner never had a chance to commit.
#[test]
fn an_untested_axis_is_unknown_rather_than_zero() {
    let score = scored();
    assert_eq!(section(&score, "02").recognition.accuracy(), None);
    assert_eq!(section(&score, "02").recognition.label(), "-");
    assert!(
        scored().report().contains("'-' means no item of that kind was asked"),
        "the table must explain the dash it prints"
    );
}

/// The table carries its own provenance, so a copied score cannot pose as a person's.
#[test]
fn the_report_says_the_session_was_synthetic() {
    let report = scored().report();
    assert!(report.contains("synthetic-2026-11-07-a"), "{report}");
    assert!(report.contains("synthetic - written, not sat"), "{report}");
    assert!(report.contains("never a person's learning"), "{report}");
}

/// Correctness follows the answers, not a stored verdict: changing one recorded answer moves
/// exactly that section's count and no other's.
#[test]
fn the_score_follows_the_answers_in_the_data() {
    let doctored = FIXTURE.replace(
        "\"answer\": \"let mut x = 5;\"",
        "\"answer\": \"let x = 5;\"",
    );
    assert_ne!(doctored, FIXTURE, "the answer to doctor must exist");
    let session = Session::from_json_str(&doctored).expect("a wrong answer is still a session");
    let score = score(&session);

    assert_eq!(section(&score, "01").attempts, Tally { tries: 4, correct: 2 });
    assert_eq!(section(&score, "02").attempts, Tally { tries: 2, correct: 0 });
    assert_eq!(score.overall.attempts, Tally { tries: 8, correct: 4 });
}

/// Loading from the path gives the same score as loading from the text: scoring reads one
/// session type, from one loader, with no second door.
#[test]
fn the_score_from_the_recorded_path_matches_the_score_from_the_text() {
    let from_disk = Session::load(FIXTURE_PATH).unwrap_or_else(|e| panic!("{FIXTURE_PATH}: {e}"));
    assert_eq!(score(&from_disk), scored());
}
