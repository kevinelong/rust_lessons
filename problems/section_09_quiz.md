# Retrieval check — section 09: Ranges and iterators

Answer from memory before looking. Each question comes from a sentence in the lesson, with one term removed.

1. The deck's "ranges may contain an infinite number of items; they save memory by generating each item as we iterate over them" is exactly what a Rust iterator is. **____** does not build a list of a thousand numbers; it is a recipe for producing them one at a time.

2. An iterator is lazy: `map` and `filter` describe work and do none of it until something consumes the chain — **____**, `sum`, `count`, `for`.

3. That laziness is why `(1..).map(...).take(5)` terminates and **____** does not.

4. How many values exceed `limit` — **____** then `count`.

5. Running totals — **____** folds and keeps each intermediate value.

<details>
<summary>Answers</summary>

1. `1..=1000`
2. `collect`
3. `(1..).collect::<Vec<>>()`
4. `filter`
5. `scan`

</details>

<sub>Generated from `src/section_09_*.rs` by `cargo run --bin quiz` — rust_lessons 0.9.0</sub>
