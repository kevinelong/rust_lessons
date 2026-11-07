# Parsons problem — section 02: Variables and data types

These are the lines of a working program from the lesson, **out of order**. Put them
back: the result should compile and behave like the example. Write your answer as a
single fenced block, then run `cargo test` — the crate has the real version.

```text
 1.     [
 2.         ("str", "&str is borrowed text, String is owned text"),
 3.     ]
 4.         ("float", "f64 by default — fractional numbers such as 0.5"),
 5. pub fn describe_types() -> [(&'static str, &'static str); 4] {
 6. }
 7.         ("bool", "true or false"),
 8.         ("int", "i32 by default — whole numbers such as 1, 2, 0, -1"),
```

<details>
<summary>Subgoal to reach for</summary>

Name the steps before you move a line: what has to exist before what. That naming is
the skill this exercise trains, not the typing.

</details>

<details>
<summary>Canonical order (do not peek first)</summary>

```rust
pub fn describe_types() -> [(&'static str, &'static str); 4] {
    [
        ("int", "i32 by default — whole numbers such as 1, 2, 0, -1"),
        ("float", "f64 by default — fractional numbers such as 0.5"),
        ("bool", "true or false"),
        ("str", "&str is borrowed text, String is owned text"),
    ]
}```

</details>

<sub>Generated from `src/section_02_*.rs` by `cargo run --bin parsons` — rust_lessons 0.17.0</sub>
