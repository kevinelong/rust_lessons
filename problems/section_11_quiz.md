# Retrieval check — section 11: Functions

Answer from memory before looking. Each question comes from a sentence in the lesson, with one term removed.

1. A function that never returns at all has the return type `!`, the never type — useful for **____**-style helpers and for a loop that can only be left by a signal.

2. The symbols in those signatures are introduced where they first appear and collected in one table in **____**: `&` and `&mut` (borrow), `->` (returns), `::` (path), `|x|` (closure), `_` (deliberately unused), `!` (never type, and the marker that makes `println!` a macro rather than a function).

3. Arguments of `&str` and **____** are borrowed, which is the closest thing Rust has to "pass by reference" and the reason none of these functions copy their input.

4. **____** would be a dependency; a linear congruential generator is forty lines of arithmetic and makes every dice exercise here reproducible, which matters when the output is a test.

5. A number in **____**, which is what a die shows.

<details>
<summary>Answers</summary>

1. `panic!`
2. `docs/symbols.md`
3. `&[i32]`
4. `rand`
5. `1..=sides`

</details>

<sub>Generated from `src/section_11_*.rs` by `cargo run --bin quiz` — rust_lessons 0.20.0</sub>
