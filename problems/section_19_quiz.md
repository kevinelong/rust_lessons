# Retrieval check — section 19: The keyboard and the disk

Answer from memory before looking. Each question comes from a sentence in the lesson, with one term removed.

1. `pythonlessons` carries `interactive.py`, **____** and a `data.txt` to read; the deck's slides never cover either.

2. stdin — **____**, one line at a time, `Ok(0)` meaning end of input.

3. Reading never panics; it returns `io::Result`. files — **____** for a whole file, `write` for output.

4. Both return **____**, and `?` is what turns four lines of error handling into one.

5. Note that the functions here take a **____** or a `path` rather than reaching for the global stdin or a hard-coded filename: that is what makes them testable, and it is the same reason section 11's functions take slices instead of vectors.

<details>
<summary>Answers</summary>

1. `09fileinputandoutput.py`
2. `std::io::stdin().readline(&mut buffer)?`
3. `std::fs::readtostring(path)?`
4. `Result`
5. `reader`

</details>

<sub>Generated from `src/section_19_*.rs` by `cargo run --bin quiz` — rust_lessons 0.12.0</sub>
