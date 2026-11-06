# Parsons problem — section 04: Strings, numbers and conversion

These are the lines of a working program from the lesson, **out of order**. Put them
back: the result should compile and behave like the example. Write your answer as a
single fenced block, then run `cargo test` — the crate has the real version.

```text
 1. pub fn parse_sum(a: &str, b: &str) -> Option<i64> {
 2.     let a: i64 = a.trim().parse().ok()?;
 3.     Some(a + b)
 4. }
 5.     let b: i64 = b.trim().parse().ok()?;
```

<details>
<summary>Subgoal to reach for</summary>

Name the steps before you move a line: what has to exist before what. That naming is
the skill this exercise trains, not the typing.

</details>

<details>
<summary>Canonical order (do not peek first)</summary>

```rust
pub fn parse_sum(a: &str, b: &str) -> Option<i64> {
    let a: i64 = a.trim().parse().ok()?;
    let b: i64 = b.trim().parse().ok()?;
    Some(a + b)
}```

</details>

<sub>Generated from `src/section_04_*.rs` by `cargo run --bin parsons` — rust_lessons 0.12.0</sub>
