# Faded Parsons problem — section 03: Operators and expressions

The **structure is given and the decisions are missing**. Fill the blanks so the function compiles and behaves like the lesson's version. Each blank is a decision point: something has to exist before that line can run, and naming that is the exercise.

```rust
pub fn operator_table() -> [(&'static str, &'static str, &'static str); 5] {
    [
        ("+", "7 + 3", "10"),
        ("-", "7 - 3", "4"),
        ("*", "7 * 3", "21"),
        ("/", "7 / 3", "2 (integer division)"),
        ("%", "7 % 3", "1"),
    ]
}
```

## The blanks


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
}
```

</details>

<sub>Generated from `src/section_03_*.rs` by `cargo run --bin parsons` — rust_lessons 0.17.0
</sub>
