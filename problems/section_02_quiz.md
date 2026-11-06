# Retrieval check — section 02: Variables and data types

Answer from memory before looking. Each question comes from a sentence in the lesson, with one term removed.

1. A variable is immutable unless you say `mut`. `let x = 5;` cannot be reassigned; **____** can.

2. A variable always has a type, and the compiler infers it from the value when you do not write it: **____** is an `i32`.

3. **____** again with the same name is shadowing, not reassignment — the second binding is a new variable that may even have a different type.

4. That is how you convert a value in place: **____**

5. **____** with two `i64` values performs integer division: the fractional part is discarded, and `(1 + 2) / 2` is `1`, not `1.5`.

<details>
<summary>Answers</summary>

1. `let mut x = 5;`
2. `let count = 3`
3. `let`
4. `let text = "42"; let text = text.parse()?;`
5. `(a + b) / 2`

</details>

<sub>Generated from `src/section_02_*.rs` by `cargo run --bin quiz` — rust_lessons 0.12.0</sub>
