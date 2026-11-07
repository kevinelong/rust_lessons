# Parsons problem — section 09: Ranges and iterators

These are the lines of a working program from the lesson, **out of order**. Put them
back: the result should compile and behave like the example. Write your answer as a
single fenced block, then run `cargo test` — the crate has the real version.

```text
 1. pub fn running_totals(values: &[i64]) -> Vec<i64> {
 2.             Some(*acc)
 3.         .iter()
 4.             *acc += v;
 5.         .collect()
 6.         })
 7.         .scan(0i64, |acc, v| {
 8. }
 9.     values
```

<details>
<summary>Subgoal to reach for</summary>

Name the steps before you move a line: what has to exist before what. That naming is
the skill this exercise trains, not the typing.

</details>

<details>
<summary>Canonical order (do not peek first)</summary>

```rust
pub fn running_totals(values: &[i64]) -> Vec<i64> {
    values
        .iter()
        .scan(0i64, |acc, v| {
            *acc += v;
            Some(*acc)
        })
        .collect()
}```

</details>

<sub>Generated from `src/section_09_*.rs` by `cargo run --bin parsons` — rust_lessons 0.20.0</sub>
