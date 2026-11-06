# Faded Parsons problem — section 01: Comments and identifiers

The **structure is given and the decisions are missing**. Fill the blanks so the function compiles and behaves like the lesson's version. Each blank is a decision point: something has to exist before that line can run, and naming that is the exercise.

```rust
pub fn is_valid_identifier(name: &str) -> bool {
    // ______  (blank 1)
    // ______  (blank 2)
        None => return false,
        Some(first) if first.is_alphabetic() || first == '_' => {}
        _ => return false,
    }
    // ______  (blank 3)
        return false;
    }
    !KEYWORDS.contains(&name)
}
```

## The blanks

1. `let mut chars = name.chars();` — what has to exist before this line runs?
2. `match chars.next() {` — what has to exist before this line runs?
3. `if !chars.all(|c| c.is_alphanumeric() || c == '_') {` — what has to exist before this line runs?

<details>
<summary>Canonical order (do not peek first)</summary>

```rust
pub fn is_valid_identifier(name: &str) -> bool {
    let mut chars = name.chars();
    match chars.next() {
        None => return false,
        Some(first) if first.is_alphabetic() || first == '_' => {}
        _ => return false,
    }
    if !chars.all(|c| c.is_alphanumeric() || c == '_') {
        return false;
    }
    !KEYWORDS.contains(&name)
}
```

</details>

<sub>Generated from `src/section_01_*.rs` by `cargo run --bin parsons` — rust_lessons 0.12.0
</sub>
