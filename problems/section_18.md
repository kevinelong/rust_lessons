# Parsons problem — section 18: Errors: panic, Result and ?

These are the lines of a working program from the lesson, **out of order**. Put them
back: the result should compile and behave like the example. Write your answer as a
single fenced block, then run `cargo test` — the crate has the real version.

```text
 1.         .checked_mul(quantity as u64)
 2.     }
 3.         return Err(OrderError::NotPositive(quantity));
 4.     }
 5.     if total > 100_000_000 {
 6. pub fn line_total(item: &str, quantity: i64) -> Result<u64, OrderError> {
 7.     Ok(total)
 8.     if quantity <= 0 {
 9.     let total = unit
10.     let unit = price_of(item).ok_or_else(|| OrderError::Unknown(item.to_string()))?;
11.         return Err(OrderError::TooLarge(total));
12. }
13.         .ok_or(OrderError::TooLarge(u64::MAX))?;
```

<details>
<summary>Subgoal to reach for</summary>

Name the steps before you move a line: what has to exist before what. That naming is
the skill this exercise trains, not the typing.

</details>

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
}```

</details>

<sub>Generated from `src/section_18_*.rs` by `cargo run --bin parsons` — rust_lessons 0.8.0</sub>
