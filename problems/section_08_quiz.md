# Retrieval check — section 08: Maps: lookup by key

Answer from memory before looking. Each question comes from a sentence in the lesson, with one term removed.

1. The rules the deck states for dictionaries hold here exactly — all keys are one type, all values are one type — and Rust adds one more: the key type must implement **____**, which is why `String` works and `f64` does not.

2. Reading a map never panics: `get` returns **____**.

3. Writing does insert or overwrite, which is why **____** is the idiom for counting.

4. Count how often each word appears — the deck's **____** problem.

5. Words are split on whitespace and lowercased, so **____** and `the` count together; punctuation is left alone, which is the honest limit of a lesson that has not met a tokenizer yet.

<details>
<summary>Answers</summary>

1. `Hash + Eq`
2. `Option<&V>`
3. `entry(...).or_insert(0)`
4. `count_word_instances`
5. `The`

</details>

<sub>Generated from `src/section_08_*.rs` by `cargo run --bin quiz` — rust_lessons 0.20.0</sub>
