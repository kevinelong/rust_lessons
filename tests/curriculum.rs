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
    for n in 1..=20u8 {
        let beats = lesson::beats(n).unwrap_or_else(|| panic!("section {n:02} has no beats"));
        assert!(beats.len() >= 3, "section {n:02} has only {} beats", beats.len());
        let spoken = beats.iter().filter(|b| lesson::needs_narration(b)).count();
        assert!(spoken >= 1, "section {n:02} has nothing to say out loud");
        total += beats.len();
    }
    assert!(total > 100, "20 sections should give more than 100 beats, got {total}");
}
