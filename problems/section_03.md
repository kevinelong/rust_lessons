# Parsons problem — section 03: Operators and expressions

These are the lines of a working program from the lesson, **out of order**. Put them
back: the result should compile and behave like the example. Write your answer as a
single fenced block, then run `cargo test` — the crate has the real version.

```text
 1.         ("%", "7 % 3", "1"),
 2.     [
 3. pub fn operator_table() -> [(&'static str, &'static str, &'static str); 5] {
 4.         ("-", "7 - 3", "4"),
 5.         ("/", "7 / 3", "2 (integer division)"),
 6.         ("+", "7 + 3", "10"),
 7. }
 8.         ("*", "7 * 3", "21"),
 9.     ]
```

<details>
<summary>Subgoal to reach for</summary>

Name the steps before you move a line: what has to exist before what. That naming is
the skill this exercise trains, not the typing.

</details>

<details>
<summary>Canonical order (do not peek first)</summary>

```rust
pub fn operator_table() -> [(&'static str, &'static str, &'static str); 5] {
    [
        ("+", "7 + 3", "10"),
        ("-", "7 - 3", "4"),
        ("*", "7 * 3", "21"),
        ("/", "7 / 3", "2 (integer division)"),
        ("%", "7 % 3", "1"),
    ]
}```

</details>

<sub>Generated from `src/section_03_*.rs` by `cargo run --bin parsons` — rust_lessons 0.8.0</sub>
