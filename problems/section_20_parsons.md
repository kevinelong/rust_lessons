# Parsons problem — section 20: Testing, and the red-green loop

These are the lines of a working program from the lesson, **out of order**. Put them
back: the result should compile and behave like the example. Write your answer as a
single fenced block, then run `cargo test` — the crate has the real version.

```text
 1.             best = (n, steps);
 2.     best
 3.     }
 4.         if steps > best.1 {
 5.         let steps = crate::section_10_loops::collatz_steps(n);
 6.     for n in candidates {
 7.     let mut best = (1, 0);
 8. }
 9.         }
10. pub fn longest_collatz(candidates: impl IntoIterator<Item = u64>) -> (u64, u32) {
```

<details>
<summary>Subgoal to reach for</summary>

Name the steps before you move a line: what has to exist before what. That naming is
the skill this exercise trains, not the typing.

</details>

<details>
<summary>Canonical order (do not peek first)</summary>

```rust
pub fn longest_collatz(candidates: impl IntoIterator<Item = u64>) -> (u64, u32) {
    let mut best = (1, 0);
    for n in candidates {
        let steps = crate::section_10_loops::collatz_steps(n);
        if steps > best.1 {
            best = (n, steps);
        }
    }
    best
}```

</details>

<sub>Generated from `src/section_20_*.rs` by `cargo run --bin parsons` — rust_lessons 0.9.0</sub>
