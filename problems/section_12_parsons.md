# Parsons problem — section 12: Capstone: guess the number

These are the lines of a working program from the lesson, **out of order**. Put them
back: the result should compile and behave like the example. Write your answer as a
single fenced block, then run `cargo test` — the crate has the real version.

```text
 1.                 Ok(Ordering::Greater)
 2.             Ordering::Less => {
 3.                 self.high = self.high.min(value.saturating_sub(1));
 4.                 Ok(Ordering::Less)
 5.     }
 6.             Ordering::Greater => {
 7.             Ordering::Equal => Err(self.guesses),
 8.             }
 9.         self.guesses += 1;
10.             }
11.                 self.low = self.low.max(value + 1);
12.         match value.cmp(&self.secret) {
13.         }
14.     pub fn guess(&mut self, value: u32) -> Verdict {
```

<details>
<summary>Subgoal to reach for</summary>

Name the steps before you move a line: what has to exist before what. That naming is
the skill this exercise trains, not the typing.

</details>

<details>
<summary>Canonical order (do not peek first)</summary>

```rust
    pub fn guess(&mut self, value: u32) -> Verdict {
        self.guesses += 1;
        match value.cmp(&self.secret) {
            Ordering::Equal => Err(self.guesses),
            Ordering::Less => {
                self.low = self.low.max(value + 1);
                Ok(Ordering::Less)
            }
            Ordering::Greater => {
                self.high = self.high.min(value.saturating_sub(1));
                Ok(Ordering::Greater)
            }
        }
    }```

</details>

<sub>Generated from `src/section_12_*.rs` by `cargo run --bin parsons` — rust_lessons 0.20.0</sub>
