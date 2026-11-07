# Retrieval check — section 07: Vectors: collections of values

Answer from memory before looking. Each question comes from a sentence in the lesson, with one term removed.

1. Decide the element type. `Vec<i32>` and **____** fail differently; the type is the first decision, not an afterthought. 2.

2. Choose the shape of the answer. `min`/`max` have no answer for an empty list, so they return **____**; `sum` does, so it returns a number.

3. Do not mutate an input you were loaned; **____** copies before it sorts for exactly this reason.

4. Everything in a vector is the same type. **____** does not compile; use an enum when the values differ in kind.

5. Indexing past the end panics, so prefer `.get(i)` when the index comes from outside your own code — it returns **____** instead of crashing.

<details>
<summary>Answers</summary>

1. `Vec<String>`
2. `Option`
3. `sorted_ascending`
4. `vec![1, "two"]`
5. `Option<&T>`

</details>

<sub>Generated from `src/section_07_*.rs` by `cargo run --bin quiz` — rust_lessons 0.20.0</sub>
