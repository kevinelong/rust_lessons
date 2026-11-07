# Retrieval check — section 21: Reading compiler errors

Answer from memory before looking. Each question comes from a sentence in the lesson, with one term removed.

1. So this section exists, and its transcripts are captured from `rustc 1.96.0`, not remembered — see **____** for the five full outputs, verbatim.

2. 1. the code (**____**) — stable and searchable; 2. the location — where the compiler concluded the problem, which is not always where the mistake is; 3. the quoted source line; 4. the span (`^^^^`) — the exact expression under discussion; 5. the note — the part beginners skip and the part that usually contains the fix.

3. E0308, mismatched types. **____**

4. E0382, use of moved value. **____** then using `name`.

5. **____** is not `Copy`: assigning moved the buffer and the old name is dead.

<details>
<summary>Answers</summary>

1. `docs/compiler-errors.md`
2. `E0308`
3. `let count: i32 = "three";`
4. `let owned = name;`
5. `String`

</details>

<sub>Generated from `src/section_21_*.rs` by `cargo run --bin quiz` — rust_lessons 0.20.0</sub>
