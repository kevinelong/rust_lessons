# Parsons problem — section 10: Loops

These are the lines of a working program from the lesson, **out of order**. Put them
back: the result should compile and behave like the example. Write your answer as a
single fenced block, then run `cargo test` — the crate has the real version.

```text
 1.         if "aeiouAEIOU".contains(c) {
 2.         }
 3.     }
 4. pub fn count_vowels(text: &str) -> usize {
 5.         }
 6.     let mut n = 0;
 7.             n += 1;
 8.         if !c.is_ascii_alphabetic() {
 9.     for c in text.chars() {
10.             continue;
11. }
12.     n
```

<details>
<summary>Subgoal to reach for</summary>

Name the steps before you move a line: what has to exist before what. That naming is
the skill this exercise trains, not the typing.

</details>

<details>
<summary>Canonical order (do not peek first)</summary>

```rust
pub fn count_vowels(text: &str) -> usize {
    let mut n = 0;
    for c in text.chars() {
        if !c.is_ascii_alphabetic() {
            continue;
        }
        if "aeiouAEIOU".contains(c) {
            n += 1;
        }
    }
    n
}```

</details>

<sub>Generated from `src/section_10_*.rs` by `cargo run --bin parsons` — rust_lessons 0.16.0</sub>
