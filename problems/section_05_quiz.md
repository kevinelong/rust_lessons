# Retrieval check — section 05: Booleans and conditionals

Answer from memory before looking. Each question comes from a sentence in the lesson, with one term removed.

1. A `bool` is `true` or **____**, and is usually produced by comparing two values rather than typed by hand: `==`, `!=`, `<`, `<=`, `>`, `>=`.

2. The deck's rule of thumb is worth restating in Rust because the compiler enforces it: the condition of an `if` must be a **____**.

3. There is no truthiness — `if count { }` is an error, and **____** is the fix.

4. Grade a score the way the deck's exercise does, with an `if`/**____** chain.

<details>
<summary>Answers</summary>

1. `false`
2. `bool`
3. `if count > 0 { }`
4. `else`

</details>

<sub>Generated from `src/section_05_*.rs` by `cargo run --bin quiz` — rust_lessons 0.16.0</sub>
