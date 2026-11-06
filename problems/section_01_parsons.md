# Parsons problem — section 01: Comments and identifiers

These are the lines of a working program from the lesson, **out of order**. Put them
back: the result should compile and behave like the example. Write your answer as a
single fenced block, then run `cargo test` — the crate has the real version.

```text
 1.         return false;
 2.     if !chars.all(|c| c.is_alphanumeric() || c == '_') {
 3.     }
 4.     !KEYWORDS.contains(&name)
 5.     let mut chars = name.chars();
 6.         _ => return false,
 7.         Some(first) if first.is_alphabetic() || first == '_' => {}
 8.     }
 9.     match chars.next() {
10. }
11.         None => return false,
12. pub fn is_valid_identifier(name: &str) -> bool {
```

<details>
<summary>Subgoal to reach for</summary>

Name the steps before you move a line: what has to exist before what. That naming is
the skill this exercise trains, not the typing.

</details>

<details>
<summary>Canonical order (do not peek first)</summary>

```rust
pub fn is_valid_identifier(name: &str) -> bool {
    let mut chars = name.chars();
    match chars.next() {
        None => return false,
        Some(first) if first.is_alphabetic() || first == '_' => {}
        _ => return false,
    }
    if !chars.all(|c| c.is_alphanumeric() || c == '_') {
        return false;
    }
    !KEYWORDS.contains(&name)
}```

</details>

<sub>Generated from `src/section_01_*.rs` by `cargo run --bin parsons` — rust_lessons 0.10.0</sub>
