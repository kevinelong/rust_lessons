# Faded Parsons problem — section 09: Ranges and iterators

The **structure is given and the decisions are missing**. Fill the blanks so the function compiles and behaves like the lesson's version. Each blank is a decision point: something has to exist before that line can run, and naming that is the exercise.

```rust
pub fn running_totals(values: &[i64]) -> Vec<i64> {
    values
        .iter()
        .scan(0i64, |acc, v| {
            *acc += v;
            Some(*acc)
        })
        .collect()
}
```

## The blanks


<details>
<summary>Canonical order (do not peek first)</summary>

```rust
pub fn running_totals(values: &[i64]) -> Vec<i64> {
    values
        .iter()
        .scan(0i64, |acc, v| {
            *acc += v;
            Some(*acc)
        })
        .collect()
}
```

</details>

<sub>Generated from `src/section_09_*.rs` by `cargo run --bin parsons` — rust_lessons 0.12.0
</sub>
