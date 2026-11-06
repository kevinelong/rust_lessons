# Faded Parsons problem — section 10: Loops

The **structure is given and the decisions are missing**. Fill the blanks so the function compiles and behaves like the lesson's version. Each blank is a decision point: something has to exist before that line can run, and naming that is the exercise.

```rust
pub fn count_vowels(text: &str) -> usize {
    // ______  (blank 1)
    // ______  (blank 2)
        // ______  (blank 3)
            continue;
        }
        // ______  (blank 4)
            n += 1;
        }
    }
    n
}
```

## The blanks

1. `let mut n = 0;` — what has to exist before this line runs?
2. `for c in text.chars() {` — what has to exist before this line runs?
3. `if !c.is_ascii_alphabetic() {` — what has to exist before this line runs?
4. `if "aeiouAEIOU".contains(c) {` — what has to exist before this line runs?

<details>
<summary>Canonical order (do not peek first)</summary>

```rust
pub fn count_vowels(text: &str) -> usize {
    let mut n = 0;
    for c in text.chars() {
        if !c.is_ascii_alphabetic() {
            continue;
        }
        if "aeiouAEIOU".contains(c) {
            n += 1;
        }
    }
    n
}
```

</details>

<sub>Generated from `src/section_10_*.rs` by `cargo run --bin parsons` — rust_lessons 0.9.0
</sub>
