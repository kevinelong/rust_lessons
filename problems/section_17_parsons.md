# Parsons problem — section 17: Ownership and moves

These are the lines of a working program from the lesson, **out of order**. Put them
back: the result should compile and behave like the example. Write your answer as a
single fenced block, then run `cargo test` — the crate has the real version.

```text
 1.         } else {
 2.         if index < self.items.len() {
 3.             None
 4.             Some(self.items.remove(index))
 5.     }
 6.         }
 7.     pub fn take(&mut self, index: usize) -> Option<String> {
```

<details>
<summary>Subgoal to reach for</summary>

Name the steps before you move a line: what has to exist before what. That naming is
the skill this exercise trains, not the typing.

</details>

<details>
<summary>Canonical order (do not peek first)</summary>

```rust
    pub fn take(&mut self, index: usize) -> Option<String> {
        if index < self.items.len() {
            Some(self.items.remove(index))
        } else {
            None
        }
    }```

</details>

<sub>Generated from `src/section_17_*.rs` by `cargo run --bin parsons` — rust_lessons 0.9.0</sub>
