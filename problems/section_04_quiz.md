# Retrieval check — section 04: Strings, numbers and conversion

Answer from memory before looking. Each question comes from a sentence in the lesson, with one term removed.

1. **____** is borrowed text — a view into someone else's characters, the type of a string literal.

2. Cheap to pass around, cannot be changed. `String` is owned text you can grow. **____**, `.pushstr()`, `format!`.

3. Conversion is explicit and can fail, so `parse` returns a **____**: text that is not a number is an error you must handle, not a silent `NaN`.

4. Uppercase text with an exclamation mark — the deck's **____** example's cousin.

5. Add two numbers written as text. **____** when either is not a number.

<details>
<summary>Answers</summary>

1. `&str`
2. `String::from("x")`
3. `Result`
4. `twice()`
5. `None`

</details>

<sub>Generated from `src/section_04_*.rs` by `cargo run --bin quiz` — rust_lessons 0.9.0</sub>
