# Faded Parsons problem — section 19: The keyboard and the disk

The **structure is given and the decisions are missing**. Fill the blanks so the function compiles and behaves like the lesson's version. Each blank is a decision point: something has to exist before that line can run, and naming that is the exercise.

```rust
pub fn write_report(path: &Path, counts: &HashMap<String, usize>) -> io::Result<usize> {
    // ______  (blank 1)
    rows.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
    // ______  (blank 2)
        .iter()
        .map(|(word, n)| format!("{n:>4} {word}\n"))
        .collect();
    fs::write(path, &body)?;
    Ok(body.len())
}
```

## The blanks

1. `let mut rows: Vec<(&String, &usize)> = counts.iter().collect();` — what has to exist before this line runs?
2. `let body: String = rows` — what has to exist before this line runs?

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
}
```

</details>

<sub>Generated from `src/section_19_*.rs` by `cargo run --bin parsons` — rust_lessons 0.17.0
</sub>
