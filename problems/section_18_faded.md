# Faded Parsons problem — section 18: Errors: panic, Result and ?

The **structure is given and the decisions are missing**. Fill the blanks so the function compiles and behaves like the lesson's version. Each blank is a decision point: something has to exist before that line can run, and naming that is the exercise.

```rust
pub fn line_total(item: &str, quantity: i64) -> Result<u64, OrderError> {
    // ______  (blank 1)
        return Err(OrderError::NotPositive(quantity));
    }
    // ______  (blank 2)
    // ______  (blank 3)
        .checked_mul(quantity as u64)
        .ok_or(OrderError::TooLarge(u64::MAX))?;
    // ______  (blank 4)
        return Err(OrderError::TooLarge(total));
    }
    Ok(total)
}
```

## The blanks

1. `if quantity <= 0 {` — what has to exist before this line runs?
2. `let unit = price_of(item).ok_or_else(|| OrderError::Unknown(item.to_string()))?;` — what has to exist before this line runs?
3. `let total = unit` — what has to exist before this line runs?
4. `if total > 100_000_000 {` — what has to exist before this line runs?

<details>
<summary>Canonical order (do not peek first)</summary>

```rust
pub fn line_total(item: &str, quantity: i64) -> Result<u64, OrderError> {
    if quantity <= 0 {
        return Err(OrderError::NotPositive(quantity));
    }
    let unit = price_of(item).ok_or_else(|| OrderError::Unknown(item.to_string()))?;
    let total = unit
        .checked_mul(quantity as u64)
        .ok_or(OrderError::TooLarge(u64::MAX))?;
    if total > 100_000_000 {
        return Err(OrderError::TooLarge(total));
    }
    Ok(total)
}
```

</details>

<sub>Generated from `src/section_18_*.rs` by `cargo run --bin parsons` — rust_lessons 0.17.0
</sub>
