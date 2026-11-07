# Retrieval check — section 04: Strings, numbers and conversion

Answer from memory before looking. Each question comes from a sentence in the lesson, with one term removed.

1. **____** is borrowed text — a view into characters someone else owns.

2. For a string literal such as `"hi"`, the owner is the compiled binary: the view lives for the whole program (**____**), which is why literals need no lifetime of their own.

3. What "borrowed" means — and how a borrow differs from a move — is section 17; here it is only a name for "a view, not the text itself". `String` is owned text you can grow. **____**, `.push_str()`, `format!`.

4. Conversion is explicit and can fail, so `parse` returns a **____**: text that is not a number is an error you must handle, not a silent `NaN`.

5. Uppercase text with an exclamation mark — the deck's **____** example's cousin.

<details>
<summary>Answers</summary>

1. `&str`
2. `&'static str`
3. `String::from("x")`
4. `Result`
5. `twice()`

</details>

<sub>Generated from `src/section_04_*.rs` by `cargo run --bin quiz` — rust_lessons 0.17.0</sub>
