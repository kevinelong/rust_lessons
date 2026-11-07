//! Cross-cutting tests: things every section must agree on.
//!
//! The per-section tests live next to their code (`src/section_*.rs`); these are the
//! invariants of the course as a whole.

use rust_lessons::{COURSE_MAP, VERSION};

/// The version is written in exactly one place, and `Cargo.toml` must agree.
///
/// The rule this enforces: one constant, so the binary, the crate and the docs
/// cannot disagree about which build produced what is on screen.
#[test]
fn cargo_toml_and_the_version_constant_agree() {
    let cargo = include_str!("../Cargo.toml");
    let declared = cargo
        .lines()
        .find_map(|l| l.trim().strip_prefix("version = "))
        .map(|v| v.trim().trim_matches('"'))
        .expect("Cargo.toml declares a version");
    assert_eq!(declared, VERSION, "Cargo.toml and src/version.rs must match");
    assert_eq!(
        VERSION.split('.').count(),
        3,
        "major.minor.patch, always three parts"
    );
}

/// The map is the course: sixteen sections, numbered 01 to 16, each titled once.
#[test]
fn the_course_map_is_complete_and_ordered() {
    assert_eq!(COURSE_MAP.len(), 16, "the deck's sixteen sections");
    for (i, (number, title)) in COURSE_MAP.iter().enumerate() {
        let expected = format!("{:02}", i + 1);
        assert_eq!(*number, expected, "sections are numbered in order");
        assert!(!title.is_empty(), "section {number} needs a title");
    }
    let mut titles: Vec<&str> = COURSE_MAP.iter().map(|(_, t)| *t).collect();
    titles.sort_unstable();
    titles.dedup();
    assert_eq!(titles.len(), 16, "no two sections share a title");
}

/// Every section is reachable, so a broken module cannot hide behind the map.
#[test]
fn every_section_answers() {
    use rust_lessons::section_01_comments::greeting;
    use rust_lessons::section_02_variables::average;
    use rust_lessons::section_03_operators::is_even;
    use rust_lessons::section_04_strings::twice;
    use rust_lessons::section_05_conditionals::classify;
    use rust_lessons::section_06_enums::Shake;
    use rust_lessons::section_07_vectors::sum_of;
    use rust_lessons::section_08_maps::word_counts;
    use rust_lessons::section_09_iterators::evens_up_to;
    use rust_lessons::section_10_loops::fizzbuzz;
    use rust_lessons::section_11_functions::reverse_me;
    use rust_lessons::section_12_capstone::play;
    use rust_lessons::section_13_structs::{Dog, Speak};
    use rust_lessons::section_14_networking::parse_url;
    use rust_lessons::section_15_client_server::{Request, Response};
    use rust_lessons::section_16_databases::Table;

    assert_eq!(greeting("Ada"), "Hello, Ada.");
    assert_eq!(average(1, 2), 1.5);
    assert!(is_even(2));
    assert_eq!(twice("ab"), "abab");
    assert_eq!(classify(95), "A");
    assert_eq!(Shake::Vanilla.price_cents(), 350);
    assert_eq!(sum_of(&[1, 2, 3]), 6);
    assert_eq!(word_counts("a a b").get("a"), Some(&2));
    assert_eq!(evens_up_to(4), vec![2, 4]);
    assert_eq!(fizzbuzz(3), ["1", "2", "Fizz"]);
    assert_eq!(reverse_me("abc"), "cba");
    assert_eq!(play(7, &[7]), Ok(1));
    assert_eq!(Dog::new("Rex").speak(), "The dog says woof");
    assert_eq!(parse_url("http://x").map(|u| u.host), Some(String::from("x")));
    assert_eq!(Request::parse("GET / HTTP/1.0\r\n\r\n").map(|r| r.path), Some(String::from("/")));
    assert_eq!(Response::ok("x").status, 200);
    assert_eq!(Table::new("t", &["a"]).len(), 0);
}

/// Every section must still produce narratable prose.
///
/// This is the drift guard for the derived formats: slides and the narrated video are
/// generated from the lesson text, so a section whose prose disappears (or turns into
/// nothing but code) would produce a silent video rather than a visible failure.
#[test]
fn every_section_has_prose_to_narrate() {
    use rust_lessons::lesson;

    let mut total = 0;
    for n in 1..=rust_lessons::SECTION_COUNT {
        let beats = lesson::beats(n).unwrap_or_else(|| panic!("section {n:02} has no beats"));
        assert!(beats.len() >= 3, "section {n:02} has only {} beats", beats.len());
        let spoken = beats.iter().filter(|b| lesson::needs_narration(b)).count();
        assert!(spoken >= 1, "section {n:02} has nothing to say out loud");
        total += beats.len();
    }
    assert!(total > 100, "20 sections should give more than 100 beats, got {total}");
}

/// The four procedural sections must name their subgoals.
///
/// Margulieux's replicated result is that subgoal labels in expository text improve later
/// problem solving; that only holds if the labels survive editing. Sections 07, 08, 11 and
/// 16 are the procedural ones — a section that loses its "Subgoals" steps fails here.
#[test]
fn procedural_sections_name_their_subgoals() {
    for n in [7u8, 8, 11, 16] {
        let beats = rust_lessons::lesson::beats(n).expect("beats");
        let text = beats
            .iter()
            .map(|b| b.text.as_str())
            .collect::<Vec<_>>()
            .join("\n")
            .to_lowercase();
        assert!(text.contains("subgoals"), "section {n:02} lost its subgoal labels");
        let steps = text.matches("**").count() / 2;
        assert!(steps >= 3, "section {n:02} names only {steps} subgoals");
    }
}

/// Every section must appear in the further-reading register.
///
/// The register is the answer to "where do I go past this lesson", and it decays silently:
/// a new section ships and nobody adds sources. This test makes the gap visible.
#[test]
fn every_section_has_further_reading() {
    let register = std::fs::read_to_string("docs/reading.md").expect("docs/reading.md exists");
    for n in 1..=rust_lessons::SECTION_COUNT {
        let row = format!("\n| {n:02} ");
        assert!(
            register.contains(&row),
            "section {n:02} has no row in docs/reading.md"
        );
    }
}

/// Each row must offer more than one line of thought, and every row must name its depth.
#[test]
fn reading_rows_state_how_deeply_the_source_was_read() {
    let register = std::fs::read_to_string("docs/reading.md").expect("docs/reading.md exists");
    let rows: Vec<&str> = register.lines().filter(|l| l.starts_with("| ") && l.contains("**")).collect();
    assert!(rows.len() >= 20, "expected the by-section table, found {} rows", rows.len());
    for row in rows {
        assert!(
            row.contains("read**") || row.contains("skimmed**") || row.contains("named**"),
            "row does not state its depth: {row}"
        );
    }
}

/// A source is marked `read` only where the note holds the evidence.
///
/// The status column is a claim about reading, and such a claim decays the moment the page is
/// edited: a citation that once had a paper behind it keeps its `read` long after the note has
/// forgotten which sentence said what. This test makes `read` mean something a reviewer can
/// check — the row carries an evidence tag, the tag resolves to a block, that block states the
/// pages actually looked at, and it holds at least one line copied out of the source. A `read`
/// with no quote fails here rather than surviving into a citation.
///
/// It also asserts the column discriminates: something must be `read` (or the register is all
/// claim and no evidence) and something must not be (or the column proves nothing).
#[test]
fn every_read_source_has_a_quoted_line() {
    let register = std::fs::read_to_string("docs/reading.md").expect("docs/reading.md exists");

    // Evidence blocks are "### E<n> — …" up to the next heading.
    let mut blocks: Vec<(String, Vec<String>)> = Vec::new();
    let mut current: Option<(String, Vec<String>)> = None;
    for line in register.lines() {
        if let Some(rest) = line.strip_prefix("### ") {
            if let Some(block) = current.take() {
                blocks.push(block);
            }
            let id = rest.split_whitespace().next().unwrap_or_default().to_string();
            current = Some((id, Vec::new()));
        } else if line.starts_with("## ") {
            if let Some(block) = current.take() {
                blocks.push(block);
            }
        } else if let Some((_, body)) = current.as_mut() {
            body.push(line.to_string());
        }
    }
    if let Some(block) = current.take() {
        blocks.push(block);
    }
    assert!(
        blocks.len() >= 5,
        "expected evidence for at least the five load-bearing sources, found {}",
        blocks.len()
    );

    // A quoted line is a blockquote long enough to be a sentence rather than a label.
    let quoted = |body: &[String]| {
        body.iter()
            .filter(|l| {
                l.trim_start()
                    .strip_prefix("> ")
                    .is_some_and(|text| text.trim().chars().count() >= 40)
            })
            .count()
    };

    let mut read: Vec<String> = Vec::new();
    for (id, body) in &blocks {
        let status = body
            .iter()
            .find_map(|l| l.trim().strip_prefix("- Status:"))
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|| panic!("evidence block {id} states no status"));
        if status != "read" {
            continue;
        }
        assert!(
            body.iter().any(|l| l.trim().starts_with("- Seen:")),
            "{id} is marked read without saying which pages were looked at"
        );
        assert!(
            quoted(body) > 0,
            "{id} is marked read and holds no quoted line - a title is not evidence"
        );
        read.push(id.clone());
    }

    assert!(
        !read.is_empty(),
        "no source is marked read: the register is all claim and no evidence"
    );
    assert!(
        read.len() < blocks.len(),
        "every source is marked read, so the status column distinguishes nothing"
    );

    // Every row that says `read` in the tables must point at a block that says `read`.
    for line in register.lines() {
        if !(line.starts_with("| ") && line.contains("**read**")) {
            continue;
        }
        let tag = line.find("[E").unwrap_or_else(|| {
            panic!("row says read but carries no evidence tag, so nothing backs it: {line}")
        });
        let rest = &line[tag + 1..];
        let end = rest
            .find(']')
            .unwrap_or_else(|| panic!("unclosed evidence tag in row: {line}"));
        let id = &rest[..end];
        assert!(
            read.iter().any(|r| r == id),
            "row points at {id}, which is not an evidence block marked read: {line}"
        );
    }

    // …and every evidence block must be cited by a row, or it documents a source nothing uses.
    for (id, _) in &blocks {
        assert!(
            register
                .lines()
                .any(|l| l.starts_with("| ") && l.contains(&format!("[{id}]"))),
            "evidence block {id} is cited by no row in the tables"
        );
    }
}
