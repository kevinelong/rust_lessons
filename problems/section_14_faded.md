# Faded Parsons problem — section 14: Networks, IP, DNS and URLs

The **structure is given and the decisions are missing**. Fill the blanks so the function compiles and behaves like the lesson's version. Each blank is a decision point: something has to exist before that line can run, and naming that is the exercise.

```rust
pub fn is_private_ipv4(addr: &str) -> bool {
    // ______  (blank 1)
        .split('.')
        .filter_map(|p| p.parse::<u8>().ok())
        .collect();
    // ______  (blank 2)
        return false;
    }
    // ______  (blank 3)
        (10, _) => true,
        (172, second) if (16..=31).contains(&second) => true,
        (192, 168) => true,
        (127, _) => true, // loopback, not private but equally not routable
        _ => false,
    }
}
```

## The blanks

1. `let octets: Vec<u8> = addr` — what has to exist before this line runs?
2. `if octets.len() != 4 {` — what has to exist before this line runs?
3. `match (octets[0], octets[1]) {` — what has to exist before this line runs?

<details>
<summary>Canonical order (do not peek first)</summary>

```rust
pub fn is_private_ipv4(addr: &str) -> bool {
    let octets: Vec<u8> = addr
        .split('.')
        .filter_map(|p| p.parse::<u8>().ok())
        .collect();
    if octets.len() != 4 {
        return false;
    }
    match (octets[0], octets[1]) {
        (10, _) => true,
        (172, second) if (16..=31).contains(&second) => true,
        (192, 168) => true,
        (127, _) => true, // loopback, not private but equally not routable
        _ => false,
    }
}
```

</details>

<sub>Generated from `src/section_14_*.rs` by `cargo run --bin parsons` — rust_lessons 0.16.0
</sub>
