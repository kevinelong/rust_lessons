# Parsons problem — section 05: Booleans and conditionals

These are the lines of a working program from the lesson, **out of order**. Put them
back: the result should compile and behave like the example. Write your answer as a
single fenced block, then run `cargo test` — the crate has the real version.

```text
 1.     if score >= 90 {
 2.     } else {
 3.     } else if score >= 70 {
 4.         "C"
 5.     } else if score >= 60 {
 6.         "B"
 7.         "A"
 8. }
 9. pub fn classify(score: u32) -> &'static str {
10.         "F"
11.         "D"
12.     }
13.     } else if score >= 80 {
```

<details>
<summary>Subgoal to reach for</summary>

Name the steps before you move a line: what has to exist before what. That naming is
the skill this exercise trains, not the typing.

</details>

<details>
<summary>Canonical order (do not peek first)</summary>

```rust
pub fn classify(score: u32) -> &'static str {
    if score >= 90 {
        "A"
    } else if score >= 80 {
        "B"
    } else if score >= 70 {
        "C"
    } else if score >= 60 {
        "D"
    } else {
        "F"
    }
}```

</details>

<sub>Generated from `src/section_05_*.rs` by `cargo run --bin parsons` — rust_lessons 0.8.0</sub>
