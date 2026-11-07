# Retrieval check — section 10: Loops

Answer from memory before looking. Each question comes from a sentence in the lesson, with one term removed.

1. The deck splits loops into two jobs: a `for` loop that visits each item of a collection, and a **____** loop that repeats until a condition is met.

2. Rust has both plus a third, **____**, for the case where the exit is somewhere in the middle of the body.

3. Two controls work in all three: `break` leaves the loop early, **____** skips to the next turn.

4. In Rust **____** can also carry a value out of a `loop`, which is how you write "search until you find it" without a flag variable.

5. The index of the first value over the limit, found with `loop` + **____**.

<details>
<summary>Answers</summary>

1. `while`
2. `loop`
3. `continue`
4. `break`
5. `break value`

</details>

<sub>Generated from `src/section_10_*.rs` by `cargo run --bin quiz` — rust_lessons 0.16.0</sub>
