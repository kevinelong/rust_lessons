# Faded Parsons problem — section 08: Maps: lookup by key

The **structure is given and the decisions are missing**. Fill the blanks so the function compiles and behaves like the lesson's version. Each blank is a decision point: something has to exist before that line can run, and naming that is the exercise.

```rust
pub fn word_counts(text: &str) -> HashMap<String, usize> {
    // ______  (blank 1)
    // ______  (blank 2)
        // ______  (blank 3)
        // ______  (blank 4)
            continue;
        }
        *counts.entry(key).or_insert(0) += 1;
    }
    counts
}
```

## The blanks

1. `let mut counts = HashMap::new();` — what has to exist before this line runs?
2. `for word in text.split_whitespace() {` — what has to exist before this line runs?
3. `let key = word.trim_matches(|c: char| !c.is_alphanumeric()).to_lowercase();` — what has to exist before this line runs?
4. `if key.is_empty() {` — what has to exist before this line runs?

<details>
<summary>Canonical order (do not peek first)</summary>

```rust
pub fn word_counts(text: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        let key = word.trim_matches(|c: char| !c.is_alphanumeric()).to_lowercase();
        if key.is_empty() {
            continue;
        }
        *counts.entry(key).or_insert(0) += 1;
    }
    counts
}
```

</details>

<sub>Generated from `src/section_08_*.rs` by `cargo run --bin parsons` — rust_lessons 0.17.0
</sub>
