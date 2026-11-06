# Parsons problem — section 07: Vectors: collections of values

These are the lines of a working program from the lesson, **out of order**. Put them
back: the result should compile and behave like the example. Write your answer as a
single fenced block, then run `cargo test` — the crate has the real version.

```text
 1. }
 2.     out
 3. pub fn range_as_vector(n: u32) -> Vec<u32> {
 4.         out.push(i);
 5.     for i in 1..=n {
 6.     }
 7.     let mut out = Vec::new();
```

<details>
<summary>Subgoal to reach for</summary>

Name the steps before you move a line: what has to exist before what. That naming is
the skill this exercise trains, not the typing.

</details>

<details>
<summary>Canonical order (do not peek first)</summary>

```rust
pub fn range_as_vector(n: u32) -> Vec<u32> {
    let mut out = Vec::new();
    for i in 1..=n {
        out.push(i);
    }
    out
}```

</details>

<sub>Generated from `src/section_07_*.rs` by `cargo run --bin parsons` — rust_lessons 0.12.0</sub>
