//! `notebook` — write one Jupyter notebook per section (the interactive rung).
//!
//! Generated from the same source as the slides and the video: the section's `//!`/`///`
//! prose becomes **markdown cells**, lesson code stays fenced inside markdown (so it
//! renders as written and is never "run" as though it compiled standalone), and each
//! notebook ends with **real executable cells** that call this crate through
//! `evcxr_jupyter`.
//!
//! Usage: `cargo run --bin notebook -- [section]` — one section, or all twenty.
//!
//! Kernel: `cargo install evcxr_jupyter --version 0.19.0 && evcxr_jupyter --install`.
//! 0.22.0 does not build against rustc 1.96 (its pinned `ra_ap_span` breaks). Execute with
//! nbclient: `nbclient` + `jupyter-client`, kernel name `rust`.
//!
//! Every generated cell prints explicitly: evcxr 0.19 does not echo a bare expression's
//! value, and an interactive lesson that shows nothing is not interactive.

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

    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let out_dir = root.join("notebooks");
    fs::create_dir_all(&out_dir).expect("create notebooks/");

    for n in sections {
        let Some(beats) = lesson::beats(n) else {
            eprintln!("section {n:02}: no beats");
            continue;
        };
        let title = title_of(n);
        let notebook = build_notebook(n, &title, &beats, &root);
        let path = out_dir.join(format!("section_{n:02}.ipynb"));
        fs::write(&path, notebook).expect("write notebook");
        println!("section {n:02}: {} beats -> {}", beats.len(), path.display());
    }
    std::process::ExitCode::SUCCESS
}

fn title_of(n: u8) -> String {
    rust_lessons::COURSE_MAP
        .iter()
        .chain(rust_lessons::TRACK_B.iter())
        .find(|(num, _)| *num == format!("{n:02}"))
        .map(|(_, t)| t.to_string())
        .unwrap_or_else(|| format!("Section {n}"))
}

/// The whole notebook as nbformat-4 JSON.
fn build_notebook(n: u8, title: &str, beats: &[Beat], root: &PathBuf) -> String {
    let mut cells: Vec<String> = Vec::new();

    cells.push(markdown(&format!(
        "# Section {n:02} — {title}\n\n\
         <sub>rust_lessons {} · generated from `src/section_{n:02}_*.rs` by `cargo run --bin notebook`</sub>",
        rust_lessons::VERSION
    )));

    for beat in beats {
        let text = beat.text.trim();
        if !text.is_empty() {
            cells.push(markdown(text));
        }
    }

    let dep = format!(
        "// The course crate, loaded from this repository. Run this cell first.\n:dep rust_lessons = {{ path = \"{}\" }}",
        root.to_string_lossy().replace('\\', "/")
    );
    cells.push(code(&dep));
    for snippet in playground(n) {
        cells.push(code(&snippet));
    }

    format!(
        "{{\n \"cells\": [\n{}\n ],\n \"metadata\": {{\n  \"kernelspec\": {{\"display_name\": \"Rust\", \"language\": \"rust\", \"name\": \"rust\"}},\n  \"language_info\": {{\"name\": \"rust\"}}\n }},\n \"nbformat\": 4,\n \"nbformat_minor\": 5\n}}\n",
        cells.join(",\n")
    )
}

fn markdown(src: &str) -> String {
    format!(
        "  {{\n   \"cell_type\": \"markdown\",\n   \"metadata\": {{}},\n   \"source\": {}\n  }}",
        json_array(src)
    )
}

fn code(src: &str) -> String {
    format!(
        "  {{\n   \"cell_type\": \"code\",\n   \"execution_count\": null,\n   \"metadata\": {{}},\n   \"outputs\": [],\n   \"source\": {}\n  }}",
        json_array(src)
    )
}

/// A JSON array of lines, the form nbformat expects for `source`.
fn json_array(src: &str) -> String {
    let trimmed = src.trim_end_matches('\n');
    if trimmed.is_empty() {
        return String::from("[]");
    }
    let total = trimmed.lines().count();
    let lines: Vec<String> = trimmed
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let newline = if i + 1 == total { "" } else { "\\n" };
            format!("\"{}{}\"", escape(line), newline)
        })
        .collect();
    format!("[{}]", lines.join(", "))
}

fn escape(s: &str) -> String {
    let mut out = String::new();
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\t' => out.push_str("\\t"),
            '\r' => {}
            c if (c as u32) < 0x20 => out.push(' '),
            c => out.push(c),
        }
    }
    out
}

/// Executable cells per section: calls into the crate the tests already exercise.
fn playground(n: u8) -> Vec<String> {
    let snippets: &[&str] = match n {
        1 => &[
            "use rust_lessons::section_01_comments::{greeting, is_valid_identifier};\nprintln!(\"{}\", greeting(\"Ada\"));",
            "println!(\"{:?}\", (is_valid_identifier(\"total_price\"), is_valid_identifier(\"2price\")));",
        ],
        2 => &[
            "use rust_lessons::section_02_variables::{average, average_without_casting};\nprintln!(\"{:?}\", (average(1, 2), average_without_casting(1, 2)));",
        ],
        3 => &[
            "use rust_lessons::section_03_operators::*;\nprintln!(\"{:?}\", (without_parentheses(2, 3, 4), grouped_differently(2, 3, 4), celsius_to_fahrenheit(100.0)));",
        ],
        4 => &[
            "use rust_lessons::section_04_strings::*;\nprintln!(\"{:?}\", (shout(\" hi \"), twice(\"na\"), parse_sum(\"2\", \"3\"), parse_sum(\"two\", \"3\"), with_separators(1234567)));",
        ],
        5 => &[
            "use rust_lessons::section_05_conditionals::{classify, sign};\nprintln!(\"{:?}\", (classify(95), classify(59), sign(-5)));",
        ],
        6 => &[
            "use rust_lessons::section_06_enums::Shake;\nprintln!(\"{:?}\", Shake::all().iter().map(|s| (s.label(), s.price_cents())).collect::<Vec<_>>());",
        ],
        7 => &[
            "use rust_lessons::section_07_vectors::*;\nlet v = [3, 2, 1, 2];\nprintln!(\"{:?}\", (min_of(&v), max_of(&v), sum_of(&v), sorted_ascending(&v)));",
        ],
        8 => &[
            "use rust_lessons::section_08_maps::*;\nlet c = word_counts(\"the cat and the hat\");\nprintln!(\"{:?}\", (count_of(&c, \"the\"), most_common(&c)));",
        ],
        9 => &[
            "use rust_lessons::section_09_iterators::*;\nprintln!(\"{:?}\", (evens_up_to(10), first_squares(5), running_totals(&[1, 2, 3])));",
        ],
        10 => &[
            "use rust_lessons::section_10_loops::*;\nprintln!(\"{:?}\", (fizzbuzz(15), collatz_steps(27), count_vowels(\"hello world\")));",
        ],
        11 => &[
            "use rust_lessons::section_11_functions::*;\nlet mut rng = Rng::new(7);\nprintln!(\"{:?}\", (roll(3, 6, &mut rng), reverse_me(\"héllo\")));",
            "use rust_lessons::section_11_functions::*;\nlet mut rng = Rng::new(2026);\nprint!(\"{}\", chart_average(&track_average(1, 6, 200, &mut rng), 20));",
        ],
        12 => &[
            "use rust_lessons::section_12_capstone::play;\nprintln!(\"{:?}\", (play(42, &[50, 25, 42]), play(42, &[1, 2, 3])));",
        ],
        13 => &[
            "use rust_lessons::section_13_structs::{Cat, Dog, Speak};\nlet dog = Dog::new(\"Rex\");\nlet cat = Cat { name: String::from(\"Mog\") };\nprintln!(\"{}\", dog.speak());\nprintln!(\"{}\", cat.speak());",
            "use rust_lessons::section_13_structs::Account;\nlet mut a = Account::new(\"Ada\");\na.deposit(500);\nprintln!(\"{:?}\", (a.withdraw(200), a.withdraw(400), a.balance()));",
        ],
        14 => &[
            "use rust_lessons::section_14_networking::*;\nlet u = parse_url(\"https://Example.com:8080/a\").unwrap();\nprintln!(\"{:?}\", (u.host.clone(), u.effective_port(), is_private_ipv4(\"10.0.0.5\")));",
        ],
        15 => &[
            "use rust_lessons::section_15_client_server::{Request, Response};\nlet r = Request::parse(\"GET /health HTTP/1.0\\r\\nHost: x\\r\\n\\r\\n\").unwrap();\nprintln!(\"{:?}\", r.path);\nprintln!(\"{}\", Response::ok(\"hi\").to_http().lines().next().unwrap());",
        ],
        16 => &[
            "use rust_lessons::section_16_databases::Table;\nlet mut t = Table::new(\"books\", &[\"id\", \"title\"]);\nt.insert(&[\"1\", \"Dune\"]).unwrap();\nprintln!(\"{:?}\", (t.select_all().len(), t.select_sql(None)));",
        ],
        17 => &[
            "use rust_lessons::section_17_ownership::{length_of, take_then_return};\nlet s = String::from(\"hello\");\nprintln!(\"{:?}\", (length_of(&s), take_then_return(s.clone()), s));",
        ],
        18 => &[
            "use rust_lessons::section_18_errors::{line_total, order_total};\nprintln!(\"{:?}\", (line_total(\"coffee\", 2), line_total(\"caviar\", 1), order_total(&[(\"tea\", 2), (\"coffee\", 1)])));",
        ],
        19 => &[
            "use std::io::Cursor;\nuse rust_lessons::section_19_input_output::*;\nlet mut c = Cursor::new(\"a\\nb\\n\");\nprintln!(\"{:?}\", read_all_lines(&mut c));",
        ],
        _ => &[
            "use rust_lessons::section_20_testing::{divide, is_palindrome};\nprintln!(\"{:?}\", (divide(10.0, 5.0), divide(1.0, 0.0), is_palindrome(\"A man, a plan, a canal: Panama\")));",
        ],
    };
    snippets.iter().map(|s| s.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cells_are_valid_json_arrays_of_lines() {
        assert_eq!(json_array("one\ntwo"), r#"["one\n", "two"]"#);
        assert_eq!(json_array(""), "[]");
        assert!(json_array("quote \" here").contains("\\\""));
    }

    #[test]
    fn every_section_has_playground_cells_that_print() {
        for n in 1..=rust_lessons::SECTION_COUNT {
            let cells = playground(n);
            assert!(!cells.is_empty(), "section {n} has no runnable cells");
            assert!(
                cells.iter().all(|c| c.contains("print")),
                "section {n} has a cell that would print nothing"
            );
        }
    }

    #[test]
    fn notebook_is_nbformat_4_and_loads_the_crate() {
        let beats = lesson::beats(1).expect("beats");
        let nb = build_notebook(1, "Comments and identifiers", &beats, &PathBuf::from("C:/repo"));
        assert!(nb.contains("\"nbformat\": 4"));
        assert!(nb.contains(":dep rust_lessons = { path = \\\"C:/repo\\\" }"));
        assert!(nb.contains("\"cell_type\": \"markdown\""));
        assert!(nb.contains("\"cell_type\": \"code\""));
    }
}
