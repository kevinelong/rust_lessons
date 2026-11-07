# 07 — Vectors: collections of values

<sub>rust_lessons 0.17.0 · source: `src/section_07_*.rs` · run: `cargo run --example 07_vectors`</sub>

Section 7 — vectors: the list, in Rust.

# Subgoals

Solving anything with a vector is the same four moves. Name them before writing code —
the labels are what the evidence says transfers (see `docs/research/2026-11-06.md`).

1. **Decide the element type.** `Vec<i32>` and `Vec<String>` fail differently; the type
   is the first decision, not an afterthought.
2. **Choose the shape of the answer.** `min`/`max` have no answer for an empty list, so
   they return `Option`; `sum` does, so it returns a number. Decide this before looping.
3. **Fold over the elements.** One traversal, one accumulator (or one running extreme).
4. **Return without surprising the caller.** Do not mutate an input you were loaned;
   `sorted_ascending` copies before it sorts for exactly this reason.
A list is a sequence of values, written `[v1, v2, v3]` in the deck's Python.
In Rust a `Vec<T>` is the growable version of that (`[T; N]` is a fixed-size
array, and slices `&[T]` are borrowed views of either).

Two rules the compiler enforces that the deck has to say out loud:

* **Everything in a vector is the same type.** `vec![1, "two"]` does not
  compile; use an enum when the values differ in kind.
* **Indexing past the end panics**, so prefer `.get(i)` when the index comes
  from outside your own code — it returns `Option<&T>` instead of crashing.

The smallest value, or `None` for an empty list.

Note the parameter: `&[i32]` accepts a `Vec<i32>` (by borrowing) *and* an
array, because both coerce to a slice. That is why library functions here take
slices rather than `Vec`.

The largest value, or `None` for an empty list.

The total, which for an empty list is 0 rather than an error.

Sort ascending, returning a new vector — `sort` mutates, so we take a copy
first rather than surprising the caller.

Index safely: the position of the first occurrence, if there is one.

Build a vector with `push`, the deck's `append` — and show why `Vec` is
declared `mut` for that to be allowed.

