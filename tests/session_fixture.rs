//! The synthetic session fixture, loaded the way scoring will load it.
//!
//! Acceptance for `measurement-fixture` in `docs/PLAN.json`: one fixture session (answers,
//! timings, a wrong-then-right pair) checked in as data, loaded by a test, with no UI and
//! no network. This file is that test, and the numbers it asserts are the numbers
//! `phase4-scoring` will have to reproduce.
//!
//! Why this exists at all: scoring a learner needs a learner. Without a recorded session,
//! every scoring change is untestable, and "we can score a session" stays a belief. The
//! fixture is synthetic — it was written, not sat — so nothing here is evidence about
//! anyone's learning, only about the reading and arithmetic of scores.

use rust_lessons::session::{Attempt, Mode, Session, SCHEMA};

const FIXTURE_PATH: &str = "fixtures/session-2026-11-07-synthetic.json";
const FIXTURE: &str = include_str!("../fixtures/session-2026-11-07-synthetic.json");

fn fixture() -> Session {
    Session::from_json_str(FIXTURE).unwrap_or_else(|e| panic!("{FIXTURE_PATH} must load: {e}"))
}

/// The data loads, and it loads to exactly one session with the metadata it claims.
#[test]
fn the_fixture_loads_as_a_session_with_no_person_in_it() {
    let session = fixture();
    assert_eq!(session.schema, SCHEMA);
    assert_eq!(session.session_id, "synthetic-2026-11-07-a");
    assert_eq!(session.learner, "synthetic-fixture");
    assert!(
        session.synthetic,
        "a session nobody sat must say so, or a score will be read as a person's"
    );
    assert_eq!(session.started, "2026-11-07T08:40:00Z");
    assert_eq!(session.items.len(), 8);
}

/// Loading from a path gives the same data as loading from the text: one reader, two
/// doors, no second parser to disagree with the first.
#[test]
fn the_fixture_loads_from_a_path_without_a_network_or_a_window() {
    let from_disk = Session::load(FIXTURE_PATH).unwrap_or_else(|e| panic!("{FIXTURE_PATH}: {e}"));
    assert_eq!(from_disk, fixture());
}

/// The pair the acceptance names: one item answered wrong, then right, recorded as two
/// rows on the same item rather than two similar items.
#[test]
fn the_fixture_holds_exactly_one_wrong_then_right_pair() {
    let session = fixture();
    let mut keys: Vec<(&str, &str)> = session.items.iter().map(|a| a.item()).collect();
    keys.sort_unstable();
    keys.dedup();

    let retried: Vec<(&str, &str)> = keys
        .into_iter()
        .filter(|(section, prompt)| session.attempts_for(section, prompt).len() > 1)
        .collect();
    assert_eq!(retried.len(), 1, "exactly one item was retried, found {retried:?}");

    let (section, prompt) = retried[0];
    assert_eq!((section, prompt), ("01", "s01-q2"));
    let attempts: Vec<&Attempt> = session.attempts_for(section, prompt);
    assert_eq!(attempts.len(), 2);
    assert_eq!(
        attempts.iter().map(|a| a.is_correct()).collect::<Vec<_>>(),
        vec![false, true],
        "wrong first, right second — not the other way round"
    );
    // And the retry is a different answer, not the same one recorded twice.
    assert_ne!(attempts[0].answer, attempts[1].answer);
    assert!(attempts[1].ms < attempts[0].ms, "the retry took less time");
}

/// Every number a phase-4 score is computed from, pinned. If scoring disagrees with any
/// of these, one of the two is wrong and this test says which.
#[test]
fn the_fixture_numbers_are_pinned_for_scoring() {
    let session = fixture();

    assert_eq!(session.sections(), vec!["01", "02", "03"]);

    let count = |mode: Mode| session.items.iter().filter(|a| a.mode == mode).count();
    assert_eq!(count(Mode::Retrieval), 6, "retrieval attempts");
    assert_eq!(count(Mode::Recognition), 2, "recognition attempts");

    let right = |mode: Mode| {
        session
            .items
            .iter()
            .filter(|a| a.mode == mode && a.is_correct())
            .count()
    };
    assert_eq!(right(Mode::Retrieval), 4, "correct retrieval attempts");
    assert_eq!(right(Mode::Recognition), 1, "correct recognition attempts");
    assert_eq!(session.items.iter().filter(|a| a.is_correct()).count(), 5);

    let total_ms: u64 = session.items.iter().map(|a| a.ms).sum();
    assert_eq!(total_ms, 102_400, "total time on task");
    assert_eq!(
        session.items.iter().map(|a| a.seconds()).sum::<f64>(),
        102.4
    );

    // Slowest first: the two Parsons problems, which is what a 36–41 s exercise should do.
    let slowest = session
        .items
        .iter()
        .max_by_key(|a| a.ms)
        .expect("a session with no attempts is not a session");
    assert_eq!(
        (slowest.section.as_str(), slowest.prompt.as_str()),
        ("01", "s01-parsons")
    );
}

/// The fixture's prompts are real: each retrieval answer is the answer the generated
/// section quiz carries for that question number, so the data cannot drift away from the
/// course and still be scored.
#[test]
fn every_retrieval_answer_is_the_one_the_section_quiz_gives() {
    let session = fixture();
    for attempt in &session.items {
        if attempt.mode != Mode::Retrieval {
            continue;
        }
        let question = attempt
            .prompt
            .split_once("-q")
            .map(|(_, n)| n)
            .unwrap_or_else(|| panic!("{} is not an sNN-qN id", attempt.prompt));
        assert!(
            attempt.prompt.starts_with(&format!("s{}-", attempt.section)),
            "{} claims section {}",
            attempt.prompt,
            attempt.section
        );
        let quiz = std::fs::read_to_string(format!(
            "problems/section_{}_quiz.md",
            attempt.section
        ))
        .unwrap_or_else(|e| panic!("section {} has no retrieval check: {e}", attempt.section));
        let (questions, answers) = quiz
            .split_once("<details>")
            .expect("a retrieval check carries its answers in a disclosure");
        assert!(
            questions.contains(&format!("\n{question}. ")),
            "section {} has no question {question}",
            attempt.section
        );
        assert!(
            answers.contains(&format!("\n{question}. `{}`", attempt.expected)),
            "section {} question {question} does not answer `{}`",
            attempt.section,
            attempt.expected
        );
    }
}

/// And each recognition answer is an ordering of the lines that section's Parsons problem
/// actually gives — not a sequence of numbers that only looks like one.
#[test]
fn every_recognition_answer_orders_the_parsons_lines_it_was_given() {
    let session = fixture();
    let mut checked = 0;
    for attempt in &session.items {
        if attempt.mode != Mode::Recognition {
            continue;
        }
        assert_eq!(
            attempt.prompt,
            format!("s{}-parsons", attempt.section),
            "a recognition item is its section's Parsons problem"
        );
        let problem = std::fs::read_to_string(format!(
            "problems/section_{}_parsons.md",
            attempt.section
        ))
        .unwrap_or_else(|e| panic!("section {} has no Parsons problem: {e}", attempt.section));
        let block = problem
            .split_once("```text")
            .and_then(|(_, rest)| rest.split_once("```"))
            .map(|(block, _)| block)
            .expect("the Parsons problem lists its lines in a text block");
        let line_count = block.lines().filter(|l| !l.trim().is_empty()).count();

        let mut order: Vec<usize> = attempt
            .expected
            .split(',')
            .map(|n| n.trim().parse::<usize>().unwrap_or_else(|_| panic!("{n:?} is not a line number")))
            .collect();
        order.sort_unstable();
        assert_eq!(
            order,
            (1..=line_count).collect::<Vec<_>>(),
            "{} must use each of the {line_count} lines exactly once",
            attempt.prompt
        );
        checked += 1;
    }
    assert_eq!(checked, 2, "both Parsons items were checked");
}

/// A zero-duration or out-of-order session is refused rather than averaged in. This is
/// the loader's promise to scoring, exercised against the real fixture text.
#[test]
fn a_doctored_fixture_is_refused() {
    let doctored = FIXTURE.replace("\"ms\": 6100", "\"ms\": 0");
    let error = Session::from_json_str(&doctored).expect_err("a zero duration is not a timing");
    assert!(error.contains("zero time"), "{error}");

    let doctored = FIXTURE.replace("\"attempt\": 2", "\"attempt\": 5");
    let error = Session::from_json_str(&doctored).expect_err("a retry cannot skip");
    assert!(error.contains("expected attempt"), "{error}");
}
