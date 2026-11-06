# Parsons problem — section 13: Structs, methods and traits

These are the lines of a working program from the lesson, **out of order**. Put them
back: the result should compile and behave like the example. Write your answer as a
single fenced block, then run `cargo test` — the crate has the real version.

```text
 1.     }
 2.                 "{} has {} cents; cannot withdraw {}",
 3.             ));
 4.     pub fn withdraw(&mut self, cents: u32) -> Result<i64, String> {
 5.             return Err(format!(
 6.         if (cents as i64) > self.cents {
 7.         }
 8.         self.cents -= cents as i64;
 9.                 self.owner, self.cents, cents
10.         Ok(self.cents)
```

<details>
<summary>Subgoal to reach for</summary>

Name the steps before you move a line: what has to exist before what. That naming is
the skill this exercise trains, not the typing.

</details>

<details>
<summary>Canonical order (do not peek first)</summary>

```rust
    pub fn withdraw(&mut self, cents: u32) -> Result<i64, String> {
        if (cents as i64) > self.cents {
            return Err(format!(
                "{} has {} cents; cannot withdraw {}",
                self.owner, self.cents, cents
            ));
        }
        self.cents -= cents as i64;
        Ok(self.cents)
    }```

</details>

<sub>Generated from `src/section_13_*.rs` by `cargo run --bin parsons` — rust_lessons 0.9.0</sub>
