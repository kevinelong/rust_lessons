# Parsons problem — section 14: Networks, IP, DNS and URLs

These are the lines of a working program from the lesson, **out of order**. Put them
back: the result should compile and behave like the example. Write your answer as a
single fenced block, then run `cargo test` — the crate has the real version.

```text
 1.         (172, second) if (16..=31).contains(&second) => true,
 2.     }
 3.         (10, _) => true,
 4. pub fn is_private_ipv4(addr: &str) -> bool {
 5.     match (octets[0], octets[1]) {
 6.     if octets.len() != 4 {
 7.         .collect();
 8.     let octets: Vec<u8> = addr
 9.         .split('.')
10.         .filter_map(|p| p.parse::<u8>().ok())
11.         (127, _) => true, // loopback, not private but equally not routable
12.     }
13.         (192, 168) => true,
14.         _ => false,
15. }
16.         return false;
```

<details>
<summary>Subgoal to reach for</summary>

Name the steps before you move a line: what has to exist before what. That naming is
the skill this exercise trains, not the typing.

</details>

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
}```

</details>

<sub>Generated from `src/section_14_*.rs` by `cargo run --bin parsons` — rust_lessons 0.8.0</sub>
