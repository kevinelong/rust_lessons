# Parsons problem — section 08: Maps: lookup by key

These are the lines of a working program from the lesson, **out of order**. Put them
back: the result should compile and behave like the example. Write your answer as a
single fenced block, then run `cargo test` — the crate has the real version.

```text
 1. pub fn word_counts(text: &str) -> HashMap<String, usize> {
 2.             continue;
 3.     counts
 4.         let key = word.trim_matches(|c: char| !c.is_alphanumeric()).to_lowercase();
 5.     }
 6.         if key.is_empty() {
 7.     let mut counts = HashMap::new();
 8. }
 9.         *counts.entry(key).or_insert(0) += 1;
10.     for word in text.split_whitespace() {
11.         }
```

<details>
<summary>Subgoal to reach for</summary>

Name the steps before you move a line: what has to exist before what. That naming is
the skill this exercise trains, not the typing.

</details>

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
}```

</details>

<sub>Generated from `src/section_08_*.rs` by `cargo run --bin parsons` — rust_lessons 0.12.0</sub>
