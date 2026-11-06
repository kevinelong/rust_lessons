# Faded Parsons problem — section 04: Strings, numbers and conversion

The **structure is given and the decisions are missing**. Fill the blanks so the function compiles and behaves like the lesson's version. Each blank is a decision point: something has to exist before that line can run, and naming that is the exercise.

```rust
pub fn parse_sum(a: &str, b: &str) -> Option<i64> {
    // ______  (blank 1)
    // ______  (blank 2)
    Some(a + b)
}
```

## The blanks

1. `let a: i64 = a.trim().parse().ok()?;` — what has to exist before this line runs?
2. `let b: i64 = b.trim().parse().ok()?;` — what has to exist before this line runs?

<details>
<summary>Canonical order (do not peek first)</summary>

```rust
pub fn parse_sum(a: &str, b: &str) -> Option<i64> {
    let a: i64 = a.trim().parse().ok()?;
    let b: i64 = b.trim().parse().ok()?;
    Some(a + b)
}
```

</details>

<sub>Generated from `src/section_04_*.rs` by `cargo run --bin parsons` — rust_lessons 0.10.0
</sub>
