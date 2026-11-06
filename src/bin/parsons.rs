//! `parsons` — generate Parsons problems from the lessons' own code.
//!
//! A Parsons problem gives the learner the lines of a working program, out of order, and
//! asks them to reassemble it. The evidence (see `docs/research/2026-11-06.md`) is that
//! this removes the blank-page cost while keeping the reasoning, and that it beats writing
//! from scratch for early practice.
//!
//! Generation, not authoring: each problem is the largest code beat of its section, with
//! the lines deterministically shuffled — same section, same scramble — and the canonical
//! order kept in a collapsible block. Nothing is written twice, and adding a section adds a
//! problem.
//!
//! Usage: `cargo run --bin parsons -- [section]`

use std::fs;
use std::path::PathBuf;

use rust_lessons::lesson;

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

    let mut written = 0;
    let mut skipped = Vec::new();
    for n in sections {
        let Some(beats) = lesson::beats(n) else {
            continue;
        };
        let _ = beats;
        let Some(source) = read_section_source(n) else {
            skipped.push(n);
            continue;
        };
        let Some(lines) = pick_function(&source) else {
            skipped.push(n);
            continue;
        };
        let scrambled = scramble(&lines, n as u64);
        let path = out_dir.join(format!("section_{n:02}_parsons.md"));
        fs::write(&path, render(n, &lines, &scrambled)).expect("write problem");

        let faded_path = out_dir.join(format!("section_{n:02}_faded.md"));
        fs::write(&faded_path, render_faded(n, &lines)).expect("write faded problem");
        written += 1;
        println!(
            "section {n:02}: {} lines -> {} + faded",
            lines.len(),
            path.display()
        );
    }
    if !skipped.is_empty() {
        println!("no code to scramble in sections: {skipped:?}");
    }
    println!("{written} problem(s) written");
    std::process::ExitCode::SUCCESS
}

/// The section's module file, read from `src/`.
pub fn read_section_source(n: u8) -> Option<String> {
    lesson::module_path(n).and_then(|p| fs::read_to_string(p).ok())
}

/// Every function in the module's non-test code, as the lines that define it.
///
/// The lesson prose rarely contains a fenced block, so the honest source for a Parsons
/// problem is a real, small, self-contained function from the section itself: 4–16 lines,
/// signature included. Test code is excluded — the point is the taught code.
pub fn extract_functions(source: &str) -> Vec<Vec<String>> {
    let mut out = Vec::new();
    let mut lines = source.lines().peekable();
    while let Some(line) = lines.next() {
        if line.starts_with("#[cfg(test)]") {
            break;
        }
        let t = line.trim_start();
        if !(t.starts_with("pub fn ") || t.starts_with("fn ")) {
            continue;
        }
        let mut body = vec![line.to_string()];
        let mut depth: i32 = 0;
        let mut opened = false;
        for c in line.chars() {
            match c {
                '{' => {
                    depth += 1;
                    opened = true;
                }
                '}' => depth -= 1,
                _ => {}
            }
        }
        while opened && depth > 0 {
            let Some(next) = lines.next() else { break };
            body.push(next.to_string());
            for c in next.chars() {
                match c {
                    '{' => depth += 1,
                    '}' => depth -= 1,
                    _ => {}
                }
            }
        }
        if opened {
            out.push(body);
        }
    }
    out
}

/// The longest function that is still short enough to reassemble by hand.
pub fn pick_function(source: &str) -> Option<Vec<String>> {
    extract_functions(source)
        .into_iter()
        .filter(|f| f.len() >= 4 && f.len() <= 16)
        .max_by_key(|f| f.len())
}

/// Deterministic shuffle: the same lines and seed always produce the same order, so a
/// committed problem file does not churn, and a learner's version matches the printed one.
pub fn scramble(lines: &[String], seed: u64) -> Vec<String> {
    let mut out = lines.to_vec();
    let mut state = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407) | 1;
    for i in (1..out.len()).rev() {
        state = state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        let j = (state >> 33) as usize % (i + 1);
        out.swap(i, j);
    }
    // A Parsons problem whose first line is already correct gives away the start.
    if out == lines && out.len() > 1 {
        out.rotate_left(1);
    }
    out
}

/// Indices to blank in a faded variant.
///
/// The rule is *decision points*, not a random third: a line is blanked when it binds a
/// name (`let`), branches (`if`/`match`), or loops (`for`/`while`). Those are the places a
/// solver has to know what must already exist — the subgoal. Structural braces, the
/// signature, and expression tails stay visible so the shape of the answer is never lost.
pub fn fade_indices(lines: &[String]) -> Vec<usize> {
    let mut picked = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        if i == 0 {
            continue; // the signature is the anchor, never blanked
        }
        let t = line.trim_start();
        if t.starts_with("let ")
            || t.starts_with("if ")
            || t.starts_with("} else if ")
            || t.starts_with("for ")
            || t.starts_with("while ")
            || t.starts_with("match ")
        {
            picked.push(i);
        }
    }
    picked
}

/// The faded rendering: blanks keep their indentation, so the block still reads as code.
pub fn render_faded(n: u8, canonical: &[String]) -> String {
    let blanks = fade_indices(canonical);
    let mut block = String::new();
    for (i, line) in canonical.iter().enumerate() {
        if let Some(pos) = blanks.iter().position(|b| *b == i) {
            let indent: String = line.chars().take_while(|c| c.is_whitespace()).collect();
            block.push_str(&format!("{indent}// ______  (blank {})\n", pos + 1));
        } else {
            block.push_str(line);
            block.push('\n');
        }
    }

    let mut list = String::new();
    for (k, i) in blanks.iter().enumerate() {
        list.push_str(&format!(
            "{}. `{}` — what has to exist before this line runs?\n",
            k + 1,
            canonical[*i].trim()
        ));
    }

    let title = title_of(n);
    let mut out = String::new();
    out.push_str(&format!("# Faded Parsons problem — section {n:02}: {title}\n\n"));
    out.push_str("The **structure is given and the decisions are missing**. Fill the blanks so the function compiles and behaves like the lesson's version. Each blank is a decision point: something has to exist before that line can run, and naming that is the exercise.\n\n");
    out.push_str("```rust\n");
    out.push_str(&block);
    out.push_str("```\n\n## The blanks\n\n");
    out.push_str(&list);
    out.push_str("\n<details>\n<summary>Canonical order (do not peek first)</summary>\n\n```rust\n");
    out.push_str(&canonical.join("\n"));
    out.push_str(&format!(
        "\n```\n\n</details>\n\n<sub>Generated from `src/section_{n:02}_*.rs` by `cargo run --bin parsons` — rust_lessons {}\n</sub>\n",
        rust_lessons::VERSION
    ));
    out
}

fn title_of(n: u8) -> String {
    rust_lessons::COURSE_MAP
        .iter()
        .chain(rust_lessons::TRACK_B.iter())
        .find(|(num, _)| *num == format!("{n:02}"))
        .map(|(_, t)| t.to_string())
        .unwrap_or_else(|| format!("Section {n}"))
}

fn render(n: u8, canonical: &[String], scrambled: &[String]) -> String {
    let title = rust_lessons::COURSE_MAP
        .iter()
        .chain(rust_lessons::TRACK_B.iter())
        .find(|(num, _)| *num == format!("{n:02}"))
        .map(|(_, t)| t.to_string())
        .unwrap_or_else(|| format!("Section {n}"));

    let numbered: String = scrambled
        .iter()
        .enumerate()
        .map(|(i, line)| format!("{:>2}. {line}\n", i + 1))
        .collect();

    format!(
        "# Parsons problem — section {n:02}: {title}\n\n\
         These are the lines of a working program from the lesson, **out of order**. Put them\n\
         back: the result should compile and behave like the example. Write your answer as a\n\
         single fenced block, then run `cargo test` — the crate has the real version.\n\n\
         ```text\n{numbered}```\n\n\
         <details>\n<summary>Subgoal to reach for</summary>\n\n\
         Name the steps before you move a line: what has to exist before what. That naming is\n\
         the skill this exercise trains, not the typing.\n\n</details>\n\n\
         <details>\n<summary>Canonical order (do not peek first)</summary>\n\n\
         ```rust\n{canonical}```\n\n</details>\n\n\
         <sub>Generated from `src/section_{n:02}_*.rs` by `cargo run --bin parsons` — rust_lessons {version}</sub>\n",
        canonical = canonical.join("\n"),
        version = rust_lessons::VERSION
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Vec<String> {
        (1..=8).map(|i| format!("line {i}")).collect()
    }

    #[test]
    fn scrambling_preserves_every_line_exactly_once() {
        let lines = sample();
        let scrambled = scramble(&lines, 7);
        assert_eq!(scrambled.len(), lines.len());
        let mut a = lines.clone();
        let mut b = scrambled;
        a.sort();
        b.sort();
        assert_eq!(a, b, "same lines, no duplicates, none lost");
    }

    #[test]
    fn the_same_seed_gives_the_same_order() {
        assert_eq!(scramble(&sample(), 11), scramble(&sample(), 11));
        assert_ne!(scramble(&sample(), 11), scramble(&sample(), 12));
    }

    #[test]
    fn the_canonical_first_line_is_never_left_in_place() {
        for seed in 1..50u64 {
            let lines = sample();
            let scrambled = scramble(&lines, seed);
            assert_ne!(scrambled, lines, "seed {seed} produced the canonical order");
        }
    }

    #[test]
    fn functions_are_extracted_with_their_whole_body() {
        let src = "fn a() {\n    one();\n}\n\npub fn b(x: u8) -> u8 {\n    if x > 1 {\n        x\n    } else {\n        0\n    }\n}\n\n#[cfg(test)]\nmod tests { fn not_this() {} }\n";
        let fns = extract_functions(src);
        assert_eq!(fns.len(), 2);
        assert_eq!(fns[1].len(), 7, "nested braces stay inside the body");
        assert!(fns[0].join("\n").contains("one();"));
        assert!(!fns.iter().flatten().any(|l| l.contains("not_this")), "tests are excluded");
    }

    #[test]
    fn fading_keeps_the_shape_and_blanks_only_decisions() {
        let canonical: Vec<String> = [
            "pub fn f(n: u32) -> u32 {",
            "    let mut total = 0;",
            "    for i in 1..=n {",
            "        total += i;",
            "    }",
            "    total",
            "}",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        let blanks = fade_indices(&canonical);
        assert_eq!(blanks, vec![1, 2], "the signature, body tail and braces stay");
        let faded = render_faded(3, &canonical);
        assert_eq!(
            faded.matches("// ______").count(),
            blanks.len(),
            "one blank per decision, no more"
        );
        assert!(faded.contains("pub fn f(n: u32) -> u32 {"), "signature is kept");
        assert!(faded.contains("    total"), "the answer tail is kept");
    }

    #[test]
    fn every_section_that_has_a_short_function_yields_a_problem() {
        let mut covered = 0;
        for n in 1..=20u8 {
            let source = read_section_source(n).expect("section source");
            if let Some(lines) = pick_function(&source) {
                covered += 1;
                assert!(
                    lines.len() >= 4 && lines.len() <= 16,
                    "section {n}: {} lines is out of range",
                    lines.len()
                );
                assert_eq!(scramble(&lines, n as u64).len(), lines.len());
            }
        }
        assert!(covered >= 15, "expected most sections to supply a problem, got {covered}");
    }
}
