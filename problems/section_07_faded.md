# Faded Parsons problem — section 07: Vectors: collections of values

The **structure is given and the decisions are missing**. Fill the blanks so the function compiles and behaves like the lesson's version. Each blank is a decision point: something has to exist before that line can run, and naming that is the exercise.

```rust
pub fn range_as_vector(n: u32) -> Vec<u32> {
    // ______  (blank 1)
    // ______  (blank 2)
        out.push(i);
    }
    out
}
```

## The blanks

1. `let mut out = Vec::new();` — what has to exist before this line runs?
2. `for i in 1..=n {` — what has to exist before this line runs?

<details>
<summary>Canonical order (do not peek first)</summary>

```rust
pub fn range_as_vector(n: u32) -> Vec<u32> {
    let mut out = Vec::new();
    for i in 1..=n {
        out.push(i);
    }
    out
}
```

</details>

<sub>Generated from `src/section_07_*.rs` by `cargo run --bin parsons` — rust_lessons 0.20.0
</sub>
