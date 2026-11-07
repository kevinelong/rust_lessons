# Retrieval check — section 20: Testing, and the red-green loop

Answer from memory before looking. Each question comes from a sentence in the lesson, with one term removed.

1. The **____** course ships `problems/` pairs and this repository carries a whole `tdd_*` family in the JavaScript cohorts — so the tooling deserves a lesson of its own rather than being something students absorb by osmosis.

2. Unit tests live beside the code they test (**____** inside the module) and can reach private functions.

3. Integration tests live in `tests/` and see only the public API — this repository's **____** is one.

4. Is **____** the same forwards and backwards, ignoring case and non-letters?

5. The largest number of steps any of **____** needs to reach 1 — used by the lesson to show a test that would take too long to run casually.

<details>
<summary>Answers</summary>

1. `python_lessons`
2. `#[cfg(test)] mod tests`
3. `tests/curriculum.rs`
4. `text`
5. `candidates`

</details>

<sub>Generated from `src/section_20_*.rs` by `cargo run --bin quiz` — rust_lessons 0.20.0</sub>
