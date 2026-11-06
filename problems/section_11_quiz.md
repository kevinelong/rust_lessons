# Retrieval check — section 11: Functions

Answer from memory before looking. Each question comes from a sentence in the lesson, with one term removed.

1. The deck's D.R.Y. slide, then the six exercises that make it real: `min`, `max`, `roll`, **____**, `trackAverage`, `chartaverage` and `reverseme`.

2. A function that never returns at all has the return type `!`, the never type — useful for **____**-style helpers and for a loop that can only be left by a signal.

3. The symbols in those signatures are introduced where they first appear and collected in one table in **____**: `&` and `&mut` (borrow), `->` (returns), `::` (path), `|x|` (closure), `` (deliberately unused), `!` (never type, and the marker that makes `println!` a macro rather than a function).

4. A Rust function declares its parameters and its return type, and the body's last expression is the return value — no **____** needed unless you are leaving early.

5. Arguments of `&str` and **____** are borrowed, which is the closest thing Rust has to "pass by reference" and the reason none of these functions copy their input.

<details>
<summary>Answers</summary>

1. `getaverageroll`
2. `panic!`
3. `docs/symbols.md`
4. `return`
5. `&[i32]`

</details>

<sub>Generated from `src/section_11_*.rs` by `cargo run --bin quiz` — rust_lessons 0.10.0</sub>
