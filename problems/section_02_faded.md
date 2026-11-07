# Faded Parsons problem — section 02: Variables and data types

The **structure is given and the decisions are missing**. Fill the blanks so the function compiles and behaves like the lesson's version. Each blank is a decision point: something has to exist before that line can run, and naming that is the exercise.

```rust
pub fn describe_types() -> [(&'static str, &'static str); 4] {
    [
        ("int", "i32 by default — whole numbers such as 1, 2, 0, -1"),
        ("float", "f64 by default — fractional numbers such as 0.5"),
        ("bool", "true or false"),
        ("str", "&str is borrowed text, String is owned text"),
    ]
}
```

## The blanks


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
}
```

</details>

<sub>Generated from `src/section_02_*.rs` by `cargo run --bin parsons` — rust_lessons 0.20.0
</sub>
