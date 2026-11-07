# Retrieval check — section 12: Capstone: guess the number

Answer from memory before looking. Each question comes from a sentence in the lesson, with one term removed.

1. This is that game, split so the logic is testable and the input handling stays in **____**.

2. The range only ever shrinks: **____** returns the smallest and largest value still possible, and each miss moves one end inward, so a game that is played sensibly cannot go on forever.

3. A new game with the secret inside the range **____**.

4. **____** means the game was won on that attempt; `Ok(Ordering)` says the guess was low or high and the player may try again.

<details>
<summary>Answers</summary>

1. `examples/12_capstone.rs`
2. `remaining_range`
3. `1..=high`
4. `Err(attempts)`

</details>

<sub>Generated from `src/section_12_*.rs` by `cargo run --bin quiz` — rust_lessons 0.17.0</sub>
