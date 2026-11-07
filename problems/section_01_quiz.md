# Retrieval check — section 01: Comments and identifiers

Answer from memory before looking. Each question comes from a sentence in the lesson, with one term removed.

1. In Rust we write names in `snake_case` — **____**, not `TotalPrice` or `total price`.

2. An identifier may contain letters, digits and underscores, but may not start with a digit, and it may not be a keyword such as **____** or `fn`.

3. A comment is a note for the reader that the compiler ignores: `//` to the end of the line, **____** for a block. `///` is a doc comment, and Rust compiles those into the documentation this course is built from.

4. Is **____** a legal Rust identifier under the rules above?

5. Comparing identifiers is case sensitive, so **____** and `total` are two different names.

<details>
<summary>Answers</summary>

1. `total_price`
2. `let`
3. `/* ... */`
4. `name`
5. `Total`

</details>

<sub>Generated from `src/section_01_*.rs` by `cargo run --bin quiz` — rust_lessons 0.16.0</sub>
