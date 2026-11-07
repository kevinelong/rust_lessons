# Faded Parsons problem — section 06: Enums: making your own type

The **structure is given and the decisions are missing**. Fill the blanks so the function compiles and behaves like the lesson's version. Each blank is a decision point: something has to exist before that line can run, and naming that is the exercise.

```rust
pub fn half_of(n: i64) -> Option<i64> {
    // ______  (blank 1)
        Some(n / 2)
    } else {
        None
    }
}
```

## The blanks

1. `if n % 2 == 0 {` — what has to exist before this line runs?

<details>
<summary>Canonical order (do not peek first)</summary>

```rust
pub fn half_of(n: i64) -> Option<i64> {
    if n % 2 == 0 {
        Some(n / 2)
    } else {
        None
    }
}
```

</details>

<sub>Generated from `src/section_06_*.rs` by `cargo run --bin parsons` — rust_lessons 0.20.0
</sub>
