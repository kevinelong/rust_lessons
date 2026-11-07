# Faded Parsons problem — section 17: Ownership and moves

The **structure is given and the decisions are missing**. Fill the blanks so the function compiles and behaves like the lesson's version. Each blank is a decision point: something has to exist before that line can run, and naming that is the exercise.

```rust
    pub fn take(&mut self, index: usize) -> Option<String> {
        // ______  (blank 1)
            Some(self.items.remove(index))
        } else {
            None
        }
    }
```

## The blanks

1. `if index < self.items.len() {` — what has to exist before this line runs?

<details>
<summary>Canonical order (do not peek first)</summary>

```rust
    pub fn take(&mut self, index: usize) -> Option<String> {
        if index < self.items.len() {
            Some(self.items.remove(index))
        } else {
            None
        }
    }
```

</details>

<sub>Generated from `src/section_17_*.rs` by `cargo run --bin parsons` — rust_lessons 0.16.0
</sub>
