# Retrieval check — section 08: Maps: lookup by key

Answer from memory before looking. Each question comes from a sentence in the lesson, with one term removed.

1. In Rust that is `HashMap<K, V>` from **____**.

2. The rules the deck states for dictionaries hold here exactly — all keys are one type, all values are one type — and Rust adds one more: the key type must implement **____**, which is why `String` works and `f64` does not.

3. Reading a map never panics: `get` returns **____**.

4. Writing does insert or overwrite, which is why **____** is the idiom for counting.

5. Count how often each word appears — the deck's **____** problem.

<details>
<summary>Answers</summary>

1. `std::collections`
2. `Hash + Eq`
3. `Option<&V>`
4. `entry(...).orinsert(0)`
5. `countwordinstances`

</details>

<sub>Generated from `src/section_08_*.rs` by `cargo run --bin quiz` — rust_lessons 0.10.0</sub>
