//! Section 19 — the keyboard and the disk.
//!
//! `python_lessons` carries `interactive.py`, `09_file_input_and_output.py` and a
//! `data.txt` to read; the deck's slides never cover either. A program that cannot read
//! a file is a program that cannot do the word-count exercise the course keeps setting,
//! so this section exists here.
//!
//! Two sources of input, one shape:
//!
//! * **stdin** — `std::io::stdin().read_line(&mut buffer)?`, one line at a time, `Ok(0)`
//!   meaning end of input. Reading never panics; it returns `io::Result`.
//! * **files** — `std::fs::read_to_string(path)?` for a whole file, `write` for output.
//!
//! Both return `Result`, and `?` is what turns four lines of error handling into one.
//! Note that the functions here take a `reader` or a `path` rather than reaching for the
//! global stdin or a hard-coded filename: that is what makes them testable, and it is
//! the same reason section 11's functions take slices instead of vectors.

use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

/// Read one line and strip the newline. `None` means end of input.
///
/// Taking `&mut impl BufRead` means a file, a string, or the real stdin all work, and a
/// test can feed a script without a keyboard.
pub fn read_line(reader: &mut impl BufRead) -> Option<String> {
    let mut line = String::new();
    match reader.read_line(&mut line) {
        Ok(0) => None,
        Ok(_) => Some(line.trim_end_matches(['\r', '\n']).to_string()),
        Err(_) => None,
    }
}

/// Every line until end of input, which is how a piped program reads `echo ... | prog`.
pub fn read_all_lines(reader: &mut impl BufRead) -> Vec<String> {
    let mut out = Vec::new();
    while let Some(line) = read_line(reader) {
        out.push(line);
    }
    out
}

/// Count the words of a file, reusing section 08's counter.
///
/// The `io::Result` is what makes a missing file a value the caller can handle rather
/// than a crash.
pub fn word_counts_in(path: &Path) -> io::Result<HashMap<String, usize>> {
    let text = fs::read_to_string(path)?;
    Ok(crate::section_08_maps::word_counts(&text))
}

/// Write a report, sorted by count then word, and return the number of bytes written.
///
/// `fs::write` creates or truncates the file and returns `io::Result<()>`.
pub fn write_report(path: &Path, counts: &HashMap<String, usize>) -> io::Result<usize> {
    let mut rows: Vec<(&String, &usize)> = counts.iter().collect();
    rows.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
    let body: String = rows
        .iter()
        .map(|(word, n)| format!("{n:>4} {word}\n"))
        .collect();
    fs::write(path, &body)?;
    Ok(body.len())
}

/// Append a line to a log file, creating it if needed — the `-a` mode of `>>`.
pub fn append_line(path: &Path, line: &str) -> io::Result<()> {
    use std::io::Write;
    let mut file = fs::OpenOptions::new().create(true).append(true).open(path)?;
    writeln!(file, "{line}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use std::path::PathBuf;

    /// A unique scratch path per test, so parallel tests cannot collide.
    fn temp_path(name: &str) -> PathBuf {
        let mut p = std::env::temp_dir();
        p.push(format!("rust_lessons_{}_{}.txt", name, std::process::id()));
        let _ = fs::remove_file(&p);
        p
    }

    #[test]
    fn lines_are_read_without_their_newlines() {
        let mut input = Cursor::new("first\r\nsecond\nthird");
        assert_eq!(read_line(&mut input).as_deref(), Some("first"));
        assert_eq!(read_line(&mut input).as_deref(), Some("second"));
        assert_eq!(read_line(&mut input).as_deref(), Some("third"));
        assert_eq!(read_line(&mut input), None, "end of input");
    }

    #[test]
    fn everything_can_be_read_at_once() {
        let mut input = Cursor::new("a\nb\n\nc");
        assert_eq!(read_all_lines(&mut input), ["a", "b", "", "c"]);
    }

    #[test]
    fn a_file_can_be_read_counted_and_written_back() {
        let src = temp_path("src");
        let out = temp_path("out");
        fs::write(&src, "the cat and the hat\n").expect("write source");

        let counts = word_counts_in(&src).expect("read back");
        assert_eq!(counts.get("the"), Some(&2));

        let bytes = write_report(&out, &counts).expect("write report");
        let report = fs::read_to_string(&out).expect("read report");
        assert_eq!(bytes, report.len());
        // Only "the" appears twice in the source, so it sorts to the top.
        assert_eq!(report.lines().next(), Some("   2 the"), "highest count first: {report:?}");

        append_line(&out, "done").expect("append");
        let after = fs::read_to_string(&out).expect("read after append");
        assert!(after.ends_with("done\n"));
        assert_eq!(
            after.lines().count(),
            counts.len() + 1,
            "one line per distinct word, plus the appended line"
        );

        let _ = fs::remove_file(&src);
        let _ = fs::remove_file(&out);
    }

    #[test]
    fn a_missing_file_is_an_error_not_a_panic() {
        let missing = temp_path("missing");
        let result = word_counts_in(&missing);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), io::ErrorKind::NotFound);
    }
}
