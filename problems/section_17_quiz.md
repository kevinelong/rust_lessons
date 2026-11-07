# Retrieval check — section 17: Ownership and moves

Answer from memory before looking. Each question comes from a sentence in the lesson, with one term removed.

1. When the owner goes out of scope the value is freed — no garbage collector, no manual **____**.

2. Moving: assigning or passing a value that owns its data (**____**, `Vec`) transfers ownership.

3. Copying: small fixed-size types (`i32`, `f64`, **____**, `char`, a fixed-size array of them) are `Copy`, so passing them copies and the original stays usable.

4. Cloning: **____** is the explicit way to keep both values, and nothing does it for you — it costs an allocation, which is exactly why `&` exists.

5. A move that goes wrong is caught at the point of use: **____** reports "use of moved value" and names both the move and the later read, so nothing has to be found at runtime.

<details>
<summary>Answers</summary>

1. `free()`
2. `String`
3. `bool`
4. `clone()`
5. `rustc`

</details>

<sub>Generated from `src/section_17_*.rs` by `cargo run --bin quiz` — rust_lessons 0.20.0</sub>
