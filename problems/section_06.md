# Parsons problem — section 06: Enums: making your own type

These are the lines of a working program from the lesson, **out of order**. Put them
back: the result should compile and behave like the example. Write your answer as a
single fenced block, then run `cargo test` — the crate has the real version.

```text
 1.     if n % 2 == 0 {
 2.     }
 3.         Some(n / 2)
 4. pub fn half_of(n: i64) -> Option<i64> {
 5. }
 6.     } else {
 7.         None
```

<details>
<summary>Subgoal to reach for</summary>

Name the steps before you move a line: what has to exist before what. That naming is
the skill this exercise trains, not the typing.

</details>

<details>
<summary>Canonical order (do not peek first)</summary>

```rust
pub fn half_of(n: i64) -> Option<i64> {
    if n % 2 == 0 {
        Some(n / 2)
    } else {
        None
    }
}```

</details>

<sub>Generated from `src/section_06_*.rs` by `cargo run --bin parsons` — rust_lessons 0.8.0</sub>
