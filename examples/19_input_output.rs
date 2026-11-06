//! Section 19, running: reading the keyboard and the disk.
//!
//! `cargo run --example 19_input_output`            (uses a temp file; no keyboard)
//! `echo -e "some words here" | cargo run --example 19_input_output`   (reads stdin)

use std::io::{self, BufRead};
use std::path::PathBuf;

use rust_lessons::section_19_input_output::{
    read_all_lines, word_counts_in, write_report,
};

fn main() {
    println!("rust_lessons {} — section 19", rust_lessons::VERSION);

    // stdin: whatever was piped in, or nothing when run from a terminal.
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let piped = io::IsTerminal::is_terminal(&handle) == false;
    if piped {
        let lines = read_all_lines(&mut handle);
        println!("  stdin: {} line(s)", lines.len());
        for line in lines.iter().take(3) {
            println!("    {line}");
        }
    } else {
        println!("  stdin: a terminal; nothing piped in.");
        // Ask for a line, the interactive way, guarded so a demo cannot block a test run.
        println!("  (type a line and press enter, or ctrl-c)");
        let mut line = String::new();
        if handle.read_line(&mut line).unwrap_or(0) > 0 {
            println!("    you said: {:?}", line.trim_end());
        }
    }

    // The disk: write a file, count its words, write a report, read the report back.
    let mut dir = std::env::temp_dir();
    dir.push(format!("rust_lessons_example_{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("create temp dir");
    let source: PathBuf = dir.join("source.txt");
    let report: PathBuf = dir.join("report.txt");

    std::fs::write(&source, "the cat sat on the mat\nthe cat ate the hat\n").expect("write source");
    let counts = word_counts_in(&source).expect("count words");
    let bytes = write_report(&report, &counts).expect("write report");

    println!("  wrote {} bytes to {}", bytes, report.display());
    print!("{}", std::fs::read_to_string(&report).expect("read report"));

    let _ = std::fs::remove_dir_all(&dir);
    println!("  (temp files removed)");
}
