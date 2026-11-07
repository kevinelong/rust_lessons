# Faded Parsons problem — section 05: Booleans and conditionals

The **structure is given and the decisions are missing**. Fill the blanks so the function compiles and behaves like the lesson's version. Each blank is a decision point: something has to exist before that line can run, and naming that is the exercise.

```rust
pub fn classify(score: u32) -> &'static str {
    // ______  (blank 1)
        "A"
    // ______  (blank 2)
        "B"
    // ______  (blank 3)
        "C"
    // ______  (blank 4)
        "D"
    } else {
        "F"
    }
}
```

## The blanks

1. `if score >= 90 {` — what has to exist before this line runs?
2. `} else if score >= 80 {` — what has to exist before this line runs?
3. `} else if score >= 70 {` — what has to exist before this line runs?
4. `} else if score >= 60 {` — what has to exist before this line runs?

<details>
<summary>Canonical order (do not peek first)</summary>

```rust
pub fn classify(score: u32) -> &'static str {
    if score >= 90 {
        "A"
    } else if score >= 80 {
        "B"
    } else if score >= 70 {
        "C"
    } else if score >= 60 {
        "D"
    } else {
        "F"
    }
}
```

</details>

<sub>Generated from `src/section_05_*.rs` by `cargo run --bin parsons` — rust_lessons 0.16.0
</sub>
