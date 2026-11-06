# Retrieval check — section 11: Functions

Answer from memory before looking. Each question comes from a sentence in the lesson, with one term removed.

1. A function that never returns at all has the return type `!`, the never type — useful for **____**-style helpers and for a loop that can only be left by a signal.

2. The symbols in those signatures are introduced where they first appear and collected in one table in **____**: `&` and `&mut` (borrow), `->` (returns), `::` (path), `|x|` (closure), `` (deliberately unused), `!` (never type, and the marker that makes `println!` a macro rather than a function).

3. A Rust function declares its parameters and its return type, and the body's last expression is the return value — no **____** needed unless you are leaving early.

4. Arguments of `&str` and **____** are borrowed, which is the closest thing Rust has to "pass by reference" and the reason none of these functions copy their input.

5. **____** would be a dependency; a linear congruential generator is forty lines of arithmetic and makes every dice exercise here reproducible, which matters when the output is a test.

<details>
<summary>Answers</summary>

1. `panic!`
2. `docs/symbols.md`
3. `return`
4. `&[i32]`
5. `rand`

</details>

<sub>Generated from `src/section_11_*.rs` by `cargo run --bin quiz` — rust_lessons 0.12.0</sub>
