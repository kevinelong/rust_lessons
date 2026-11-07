# Parsons problem — section 11: Functions

These are the lines of a working program from the lesson, **out of order**. Put them
back: the result should compile and behave like the example. Write your answer as a
single fenced block, then run `cargo test` — the crate has the real version.

```text
 1.         .into_iter()
 2.         .enumerate()
 3.     }
 4.         for _ in 0..quantity {
 5.         .collect()
 6. pub fn track_average(quantity: u32, sides: u32, times: u32, rng: &mut Rng) -> Vec<(u32, u32)> {
 7.     for _ in 0..times {
 8.             buckets[face as usize - 1] += 1;
 9.     let mut buckets = vec![0u32; sides as usize];
10.             let face = rng.roll_die(sides);
11.     buckets
12.         .map(|(i, n)| (i as u32 + 1, n))
13. }
14.         }
```

<details>
<summary>Subgoal to reach for</summary>

Name the steps before you move a line: what has to exist before what. That naming is
the skill this exercise trains, not the typing.

</details>

<details>
<summary>Canonical order (do not peek first)</summary>

```rust
pub fn track_average(quantity: u32, sides: u32, times: u32, rng: &mut Rng) -> Vec<(u32, u32)> {
    let mut buckets = vec![0u32; sides as usize];
    for _ in 0..times {
        for _ in 0..quantity {
            let face = rng.roll_die(sides);
            buckets[face as usize - 1] += 1;
        }
    }
    buckets
        .into_iter()
        .enumerate()
        .map(|(i, n)| (i as u32 + 1, n))
        .collect()
}```

</details>

<sub>Generated from `src/section_11_*.rs` by `cargo run --bin parsons` — rust_lessons 0.20.0</sub>
