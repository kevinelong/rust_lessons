# Retrieval check — section 18: Errors: panic, Result and ?

Answer from memory before looking. Each question comes from a sentence in the lesson, with one term removed.

1. Section 18 — errors: **____**, `panic!`, `Result` and `?`.

2. `python_lessons` teaches this as **____**; the deck itself has no slide for it.

3. A domain error, written out — the Rust answer to a `try`/**____**.

4. Price **____** of `item`, refusing the three things that can be wrong.

5. **____** is the idiom for "all or nothing": it returns the first error rather than a partially built list.

<details>
<summary>Answers</summary>

1. `assert`
2. `assert_try_except.py`
3. `except ValueError`
4. `quantity`
5. `collect::<Result<Vec<_>, _>>()`

</details>

<sub>Generated from `src/section_18_*.rs` by `cargo run --bin quiz` — rust_lessons 0.16.0</sub>
