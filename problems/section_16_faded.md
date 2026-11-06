# Faded Parsons problem — section 16: Tables and SQL

The **structure is given and the decisions are missing**. Fill the blanks so the function compiles and behaves like the lesson's version. Each blank is a decision point: something has to exist before that line can run, and naming that is the exercise.

```rust
    pub fn insert(&mut self, row: &[&str]) -> Result<usize, String> {
        // ______  (blank 1)
            return Err(format!(
                "{} has {} columns, row has {}",
                self.name,
                self.columns.len(),
                row.len()
            ));
        }
        self.rows.push(row.iter().map(|v| v.to_string()).collect());
        Ok(self.rows.len())
    }
```

## The blanks

1. `if row.len() != self.columns.len() {` — what has to exist before this line runs?

<details>
<summary>Canonical order (do not peek first)</summary>

```rust
    pub fn insert(&mut self, row: &[&str]) -> Result<usize, String> {
        if row.len() != self.columns.len() {
            return Err(format!(
                "{} has {} columns, row has {}",
                self.name,
                self.columns.len(),
                row.len()
            ));
        }
        self.rows.push(row.iter().map(|v| v.to_string()).collect());
        Ok(self.rows.len())
    }
```

</details>

<sub>Generated from `src/section_16_*.rs` by `cargo run --bin parsons` — rust_lessons 0.12.0
</sub>
