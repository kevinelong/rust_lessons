# Retrieval check — section 07: Vectors: collections of values

Answer from memory before looking. Each question comes from a sentence in the lesson, with one term removed.

1. A list is a sequence of values, written **____** in the deck's Python.

2. In Rust a **____** is the growable version of that (`[T; N]` is a fixed-size array, and slices `&[T]` are borrowed views of either).

3. Everything in a vector is the same type. **____** does not compile; use an enum when the values differ in kind.

4. Indexing past the end panics, so prefer `.get(i)` when the index comes from outside your own code — it returns **____** instead of crashing.

5. The smallest value, or **____** for an empty list.

<details>
<summary>Answers</summary>

1. `[v1, v2, v3]`
2. `Vec<T>`
3. `vec![1, "two"]`
4. `Option<&T>`
5. `None`

</details>

<sub>Generated from `src/section_07_*.rs` by `cargo run --bin quiz` — rust_lessons 0.10.0</sub>
