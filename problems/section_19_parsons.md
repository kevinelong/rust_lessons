# Parsons problem — section 19: The keyboard and the disk

These are the lines of a working program from the lesson, **out of order**. Put them
back: the result should compile and behave like the example. Write your answer as a
single fenced block, then run `cargo test` — the crate has the real version.

```text
 1.         .iter()
 2. pub fn write_report(path: &Path, counts: &HashMap<String, usize>) -> io::Result<usize> {
 3.         .map(|(word, n)| format!("{n:>4} {word}\n"))
 4.         .collect();
 5. }
 6.     let body: String = rows
 7.     fs::write(path, &body)?;
 8.     rows.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
 9.     let mut rows: Vec<(&String, &usize)> = counts.iter().collect();
10.     Ok(body.len())
```

<details>
<summary>Subgoal to reach for</summary>

Name the steps before you move a line: what has to exist before what. That naming is
the skill this exercise trains, not the typing.

</details>

<details>
<summary>Canonical order (do not peek first)</summary>

```rust
pub fn write_report(path: &Path, counts: &HashMap<String, usize>) -> io::Result<usize> {
    let mut rows: Vec<(&String, &usize)> = counts.iter().collect();
    rows.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
    let body: String = rows
        .iter()
        .map(|(word, n)| format!("{n:>4} {word}\n"))
        .collect();
    fs::write(path, &body)?;
    Ok(body.len())
}```

</details>

<sub>Generated from `src/section_19_*.rs` by `cargo run --bin parsons` — rust_lessons 0.16.0</sub>
