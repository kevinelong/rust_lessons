# Faded Parsons problem — section 11: Functions

The **structure is given and the decisions are missing**. Fill the blanks so the function compiles and behaves like the lesson's version. Each blank is a decision point: something has to exist before that line can run, and naming that is the exercise.

```rust
pub fn track_average(quantity: u32, sides: u32, times: u32, rng: &mut Rng) -> Vec<(u32, u32)> {
    // ______  (blank 1)
    // ______  (blank 2)
        // ______  (blank 3)
            // ______  (blank 4)
            buckets[face as usize - 1] += 1;
        }
    }
    buckets
        .into_iter()
        .enumerate()
        .map(|(i, n)| (i as u32 + 1, n))
        .collect()
}
```

## The blanks

1. `let mut buckets = vec![0u32; sides as usize];` — what has to exist before this line runs?
2. `for _ in 0..times {` — what has to exist before this line runs?
3. `for _ in 0..quantity {` — what has to exist before this line runs?
4. `let face = rng.roll_die(sides);` — what has to exist before this line runs?

<details>
<summary>Canonical order (do not peek first)</summary>

```rust
pub fn track_average(quantity: u32, sides: u32, times: u32, rng: &mut Rng) -> Vec<(u32, u32)> {
    let mut buckets = vec![0u32; sides as usize];
    for _ in 0..times {
        for _ in 0..quantity {
            let face = rng.roll_die(sides);
            buckets[face as usize - 1] += 1;
        }
    }
    buckets
        .into_iter()
        .enumerate()
        .map(|(i, n)| (i as u32 + 1, n))
        .collect()
}
```

</details>

<sub>Generated from `src/section_11_*.rs` by `cargo run --bin parsons` — rust_lessons 0.17.0
</sub>
