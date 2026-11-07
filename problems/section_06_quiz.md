# Retrieval check — section 06: Enums: making your own type

Answer from memory before looking. Each question comes from a sentence in the lesson, with one term removed.

1. The deck's **____** slide: an enumeration defines a type whose value is one of a named set.

2. Rust's `enum` is the strongest version of this idea in any of the course languages, because the variants can carry data and because **____** must be exhaustive — forget a case and the code does not compile.

3. This is where the design instinct to use a set of words instead of a magic number starts: **____**, not `0`.

4. An enum whose variants carry different data — the feature Python's **____** does not have and the reason Rust code prefers enums to class hierarchies.

5. **____** is the built-in enum you have already been using: either `Some` value or `None`.

<details>
<summary>Answers</summary>

1. `Shake(Enum)`
2. `match`
3. `Shake::Vanilla`
4. `Enum`
5. `Option<T>`

</details>

<sub>Generated from `src/section_06_*.rs` by `cargo run --bin quiz` — rust_lessons 0.17.0</sub>
