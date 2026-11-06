# 19 — The keyboard and the disk

<sub>rust_lessons 0.12.0 · source: `src/section_19_*.rs` · run: `cargo run --example 19_input_output`</sub>

Section 19 — the keyboard and the disk.

`python_lessons` carries `interactive.py`, `09_file_input_and_output.py` and a
`data.txt` to read; the deck's slides never cover either. A program that cannot read
a file is a program that cannot do the word-count exercise the course keeps setting,
so this section exists here.

Two sources of input, one shape:

* **stdin** — `std::io::stdin().read_line(&mut buffer)?`, one line at a time, `Ok(0)`
  meaning end of input. Reading never panics; it returns `io::Result`.
* **files** — `std::fs::read_to_string(path)?` for a whole file, `write` for output.

Both return `Result`, and `?` is what turns four lines of error handling into one.
Note that the functions here take a `reader` or a `path` rather than reaching for the
global stdin or a hard-coded filename: that is what makes them testable, and it is
the same reason section 11's functions take slices instead of vectors.

Read one line and strip the newline. `None` means end of input.

Taking `&mut impl BufRead` means a file, a string, or the real stdin all work, and a
test can feed a script without a keyboard.

Every line until end of input, which is how a piped program reads `echo ... | prog`.

Count the words of a file, reusing section 08's counter.

The `io::Result` is what makes a missing file a value the caller can handle rather
than a crash.

Write a report, sorted by count then word, and return the number of bytes written.

`fs::write` creates or truncates the file and returns `io::Result<()>`.

Append a line to a log file, creating it if needed — the `-a` mode of `>>`.

