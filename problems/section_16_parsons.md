# Parsons problem — section 16: Tables and SQL

These are the lines of a working program from the lesson, **out of order**. Put them
back: the result should compile and behave like the example. Write your answer as a
single fenced block, then run `cargo test` — the crate has the real version.

```text
 1.         self.rows.push(row.iter().map(|v| v.to_string()).collect());
 2.         Ok(self.rows.len())
 3.         }
 4.                 self.name,
 5.                 row.len()
 6.             return Err(format!(
 7.     }
 8.                 "{} has {} columns, row has {}",
 9.                 self.columns.len(),
10.             ));
11.         if row.len() != self.columns.len() {
12.     pub fn insert(&mut self, row: &[&str]) -> Result<usize, String> {
```

<details>
<summary>Subgoal to reach for</summary>

Name the steps before you move a line: what has to exist before what. That naming is
the skill this exercise trains, not the typing.

</details>

<details>
<summary>Canonical order (do not peek first)</summary>

```rust
    pub fn insert(&mut self, row: &[&str]) -> Result<usize, String> {
        if row.len() != self.columns.len() {
            return Err(format!(
                "{} has {} columns, row has {}",
                self.name,
                self.columns.len(),
                row.len()
            ));
        }
        self.rows.push(row.iter().map(|v| v.to_string()).collect());
        Ok(self.rows.len())
    }```

</details>

<sub>Generated from `src/section_16_*.rs` by `cargo run --bin parsons` — rust_lessons 0.16.0</sub>
