# Retrieval check — section 07: Vectors: collections of values

Answer from memory before looking. Each question comes from a sentence in the lesson, with one term removed.

1. Decide the element type. `Vec<i32>` and **____** fail differently; the type is the first decision, not an afterthought. 2.

2. Choose the shape of the answer. `min`/`max` have no answer for an empty list, so they return **____**; `sum` does, so it returns a number.

3. Do not mutate an input you were loaned; **____** copies before it sorts for exactly this reason.

4. A list is a sequence of values, written **____** in the deck's Python.

5. In Rust a **____** is the growable version of that (`[T; N]` is a fixed-size array, and slices `&[T]` are borrowed views of either).

<details>
<summary>Answers</summary>

1. `Vec<String>`
2. `Option`
3. `sortedascending`
4. `[v1, v2, v3]`
5. `Vec<T>`

</details>

<sub>Generated from `src/section_07_*.rs` by `cargo run --bin quiz` — rust_lessons 0.12.0</sub>
