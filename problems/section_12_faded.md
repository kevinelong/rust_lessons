# Faded Parsons problem — section 12: Capstone: guess the number

The **structure is given and the decisions are missing**. Fill the blanks so the function compiles and behaves like the lesson's version. Each blank is a decision point: something has to exist before that line can run, and naming that is the exercise.

```rust
    pub fn guess(&mut self, value: u32) -> Verdict {
        self.guesses += 1;
        // ______  (blank 1)
            Ordering::Equal => Err(self.guesses),
            Ordering::Less => {
                self.low = self.low.max(value + 1);
                Ok(Ordering::Less)
            }
            Ordering::Greater => {
                self.high = self.high.min(value.saturating_sub(1));
                Ok(Ordering::Greater)
            }
        }
    }
```

## The blanks

1. `match value.cmp(&self.secret) {` — what has to exist before this line runs?

<details>
<summary>Canonical order (do not peek first)</summary>

```rust
    pub fn guess(&mut self, value: u32) -> Verdict {
        self.guesses += 1;
        match value.cmp(&self.secret) {
            Ordering::Equal => Err(self.guesses),
            Ordering::Less => {
                self.low = self.low.max(value + 1);
                Ok(Ordering::Less)
            }
            Ordering::Greater => {
                self.high = self.high.min(value.saturating_sub(1));
                Ok(Ordering::Greater)
            }
        }
    }
```

</details>

<sub>Generated from `src/section_12_*.rs` by `cargo run --bin parsons` — rust_lessons 0.10.0
</sub>
