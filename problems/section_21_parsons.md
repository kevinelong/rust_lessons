# Parsons problem — section 21: Reading compiler errors

These are the lines of a working program from the lesson, **out of order**. Put them
back: the result should compile and behave like the example. Write your answer as a
single fenced block, then run `cargo test` — the crate has the real version.

```text
 1.     }
 2.         "chocolate" => Some(400),
 3.         "vanilla" => Some(350),
 4.     match shake {
 5. }
 6.         _ => None, // an unknown name is not a variant: `Option` says so
 7. pub fn price(shake: &str) -> Option<u32> {
 8.         "strawberry" => Some(375),
```

<details>
<summary>Subgoal to reach for</summary>

Name the steps before you move a line: what has to exist before what. That naming is
the skill this exercise trains, not the typing.

</details>

<details>
<summary>Canonical order (do not peek first)</summary>

```rust
pub fn price(shake: &str) -> Option<u32> {
    match shake {
        "vanilla" => Some(350),
        "chocolate" => Some(400),
        "strawberry" => Some(375),
        _ => None, // an unknown name is not a variant: `Option` says so
    }
}```

</details>

<sub>Generated from `src/section_21_*.rs` by `cargo run --bin parsons` — rust_lessons 0.12.0</sub>
