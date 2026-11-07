# 20 — Testing, and the red-green loop

<sub>rust_lessons 0.20.0 · source: `src/section_20_*.rs` · run: `cargo run --example 20_testing`</sub>

Section 20 — testing, the way the course has been graded all along.

Every section so far ended in tests, and no section explained them. The `python_lessons`
course ships `problems/` pairs and this repository carries a whole `tdd_*` family in
the JavaScript cohorts — so the tooling deserves a lesson of its own rather than being
something students absorb by osmosis.

The parts:

| Piece | Meaning |
|---|---|
| `#[test]` | marks a function as a test; it takes no arguments and returns nothing |
| `assert!(cond)` | fails with no message |
| `assert_eq!(a, b)` | fails and shows both sides — always prefer this |
| `assert_eq!(a, b, "why")` | the message a future reader needs |
| `#[should_panic(expected = "…")]` | the test passes only if it panics with that text |
| `#[ignore]` | written but not run by default; run with `cargo test -- --ignored` |

**Unit tests** live beside the code they test (`#[cfg(test)] mod tests` inside the
module) and can reach private functions. **Integration tests** live in `tests/` and see
only the public API — this repository's `tests/curriculum.rs` is one. The split is a
design pressure: if something is hard to test from outside, the public API is wrong.

The loop the course uses: write the test, watch it fail for the right reason, make it
pass, then keep it. A test that has never failed has never been proven.

A tiny function to test, so the lesson has something honest to operate on.

Is `text` the same forwards and backwards, ignoring case and non-letters?

The largest number of steps any of `candidates` needs to reach 1 — used by the lesson
to show a test that would take too long to run casually.

