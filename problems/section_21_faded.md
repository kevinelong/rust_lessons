# Faded Parsons problem — section 21: Reading compiler errors

The **structure is given and the decisions are missing**. Fill the blanks so the function compiles and behaves like the lesson's version. Each blank is a decision point: something has to exist before that line can run, and naming that is the exercise.

```rust
pub fn price(shake: &str) -> Option<u32> {
    // ______  (blank 1)
        "vanilla" => Some(350),
        "chocolate" => Some(400),
        "strawberry" => Some(375),
        _ => None, // an unknown name is not a variant: `Option` says so
    }
}
```

## The blanks

1. `match shake {` — what has to exist before this line runs?

<details>
<summary>Canonical order (do not peek first)</summary>

```rust
pub fn price(shake: &str) -> Option<u32> {
    match shake {
        "vanilla" => Some(350),
        "chocolate" => Some(400),
        "strawberry" => Some(375),
        _ => None, // an unknown name is not a variant: `Option` says so
    }
}
```

</details>

<sub>Generated from `src/section_21_*.rs` by `cargo run --bin parsons` — rust_lessons 0.12.0
</sub>
