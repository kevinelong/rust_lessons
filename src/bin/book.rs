//! `book` — write the course as an mdBook.
//!
//! Fourth rendering of the same source. Every chapter is the section module's own prose
//! (`//!` and `///`), verbatim, with a header that names the section and links the runnable
//! example. `mdbook test` then compiles every `rust` block in the book, which is a second
//! opinion on the code the tests already cover.
//!
//! Usage: `cargo run --bin book` → `book/src/*.md` + `book/src/SUMMARY.md`, then:
//!
//! ```text
//! mdbook build book      # static site in book/book/
//! mdbook test  book      # compile the code blocks
//! mdbook serve book      # read it locally
//! ```

use std::fs;
use std::path::PathBuf;

use rust_lessons::lesson;

fn main() -> std::process::ExitCode {
    let src = PathBuf::from("book/src");
    fs::create_dir_all(&src).expect("create book/src");

    // Front matter: what the book is, and how its four renderings relate.
    fs::write(src.join("introduction.md"), intro_text()).expect("write introduction");

    let mut summary = String::from("# Summary\n\n- [Introduction](introduction.md)\n");

    let mut written = 0;
    for n in 1..=rust_lessons::SECTION_COUNT {
        let Some(beats) = lesson::beats(n) else {
            continue;
        };
        let title = title_of(n);
        let example = example_name(n);
        let mut chapter = String::new();
        chapter.push_str(&format!("# {n:02} — {title}\n\n"));
        chapter.push_str(&format!(
            "<sub>rust_lessons {} · source: `src/section_{n:02}_*.rs` · run: `cargo run --example {example}`</sub>\n\n",
            rust_lessons::VERSION
        ));
        for beat in &beats {
            chapter.push_str(beat.text.trim());
            chapter.push_str("\n\n");
        }
        fs::write(src.join(format!("section_{n:02}.md")), chapter).expect("write chapter");
        summary.push_str(&format!(
            "- [{n:02} — {title}](section_{n:02}.md)\n"
        ));
        written += 1;
    }

    fs::write(src.join("SUMMARY.md"), summary).expect("write SUMMARY");
    println!("{written} chapters -> {}", src.display());
    println!("next: mdbook build book && mdbook test book");
    std::process::ExitCode::SUCCESS
}

/// The book's front matter. A raw string, so the embedded Rust block needs no escaping.
const INTRO: &str = r#"# Introduction

This is the {VERSION} edition of a course that teaches beginning programming in Rust,
lesson for lesson from a Python deck of 85 slides. The same lessons also exist as slides,
Jupyter notebooks and narrated video — all generated from the text you are reading, so they
cannot drift from it.

## How to read it

Each chapter is one section. Read it, run its example, then do its three practice artefacts:
a Parsons problem, a faded variant, and a retrieval check.

```text
cargo run --example 07_vectors      # the lesson, running
cat problems/section_07_parsons.md  # reassemble the lines
cat problems/section_07_quiz.md     # answer from memory first
```

## What is verified, and what is not

The code in the chapters is compiled by `cargo test`. Chapters also quote code as
`text` on purpose — it is there to be read, and it is already compiled elsewhere. The one
`rust` block below exists so that `mdbook test` compiles something rather than passing while
checking nothing.

```rust
fn greeting(name: &str) -> String {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        String::from("Hello, stranger.")
    } else {
        format!("Hello, {trimmed}.")
    }
}

fn main() {
    assert_eq!(greeting("Ada"), "Hello, Ada.");
    assert_eq!(greeting("   "), "Hello, stranger.");
}
```

The *teaching* is not verified: there is no measurement of a learner, and this course does not
claim one. `docs/STATE.md` carries the honest list.
"#;

fn intro_text() -> String {
    INTRO.replace("{VERSION}", rust_lessons::VERSION)
}

fn title_of(n: u8) -> String {
    rust_lessons::COURSE_MAP
        .iter()
        .chain(rust_lessons::TRACK_B.iter())
        .find(|(num, _)| *num == format!("{n:02}"))
        .map(|(_, t)| t.to_string())
        .unwrap_or_else(|| format!("Section {n}"))
}

/// The example file for a section, if one exists — the book links it by name.
fn example_name(n: u8) -> String {
    let dir = PathBuf::from("examples");
    let prefix = format!("{n:02}");
    let mut found: Vec<String> = fs::read_dir(&dir)
        .map(|d| {
            d.filter_map(|e| e.ok())
                .map(|e| e.file_name().to_string_lossy().to_string())
                .filter(|f| f.starts_with(&prefix) && f.ends_with(".rs"))
                .collect()
        })
        .unwrap_or_default();
    found.sort();
    found
        .first()
        .map(|f| f.trim_end_matches(".rs").to_string())
        .unwrap_or_else(|| format!("{prefix}_lesson"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_section_has_a_title_and_an_example_name() {
        for n in 1..=rust_lessons::SECTION_COUNT {
            assert!(!title_of(n).is_empty(), "section {n} has no title");
            let ex = example_name(n);
            assert!(!ex.is_empty());
            assert!(!ex.ends_with(".rs"), "example name is a stem, not a file: {ex}");
        }
    }

    #[test]
    fn known_examples_are_found_by_prefix() {
        assert_eq!(example_name(7), "07_vectors");
        assert_eq!(example_name(21), "21_compiler_errors");
    }
}
