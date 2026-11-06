# Faded Parsons problem — section 13: Structs, methods and traits

The **structure is given and the decisions are missing**. Fill the blanks so the function compiles and behaves like the lesson's version. Each blank is a decision point: something has to exist before that line can run, and naming that is the exercise.

```rust
    pub fn withdraw(&mut self, cents: u32) -> Result<i64, String> {
        // ______  (blank 1)
            return Err(format!(
                "{} has {} cents; cannot withdraw {}",
                self.owner, self.cents, cents
            ));
        }
        self.cents -= cents as i64;
        Ok(self.cents)
    }
```

## The blanks

1. `if (cents as i64) > self.cents {` — what has to exist before this line runs?

<details>
<summary>Canonical order (do not peek first)</summary>

```rust
    pub fn withdraw(&mut self, cents: u32) -> Result<i64, String> {
        if (cents as i64) > self.cents {
            return Err(format!(
                "{} has {} cents; cannot withdraw {}",
                self.owner, self.cents, cents
            ));
        }
        self.cents -= cents as i64;
        Ok(self.cents)
    }
```

</details>

<sub>Generated from `src/section_13_*.rs` by `cargo run --bin parsons` — rust_lessons 0.9.0
</sub>
