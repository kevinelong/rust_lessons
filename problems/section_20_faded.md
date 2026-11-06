# Faded Parsons problem — section 20: Testing, and the red-green loop

The **structure is given and the decisions are missing**. Fill the blanks so the function compiles and behaves like the lesson's version. Each blank is a decision point: something has to exist before that line can run, and naming that is the exercise.

```rust
pub fn longest_collatz(candidates: impl IntoIterator<Item = u64>) -> (u64, u32) {
    // ______  (blank 1)
    // ______  (blank 2)
        // ______  (blank 3)
        // ______  (blank 4)
            best = (n, steps);
        }
    }
    best
}
```

## The blanks

1. `let mut best = (1, 0);` — what has to exist before this line runs?
2. `for n in candidates {` — what has to exist before this line runs?
3. `let steps = crate::section_10_loops::collatz_steps(n);` — what has to exist before this line runs?
4. `if steps > best.1 {` — what has to exist before this line runs?

<details>
<summary>Canonical order (do not peek first)</summary>

```rust
pub fn longest_collatz(candidates: impl IntoIterator<Item = u64>) -> (u64, u32) {
    let mut best = (1, 0);
    for n in candidates {
        let steps = crate::section_10_loops::collatz_steps(n);
        if steps > best.1 {
            best = (n, steps);
        }
    }
    best
}
```

</details>

<sub>Generated from `src/section_20_*.rs` by `cargo run --bin parsons` — rust_lessons 0.10.0
</sub>
