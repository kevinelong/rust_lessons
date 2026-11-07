# 08 — Maps: lookup by key

<sub>rust_lessons 0.16.0 · source: `src/section_08_*.rs` · run: `cargo run --example 08_maps`</sub>

Section 8 — maps: looking a value up by another value.

# Subgoals

1. **Name the key type.** The key must be `Hash + Eq`; that is why `String` works and
   `f64` does not, and it is a decision made before any code.
2. **Decide what a missing key means.** `get` returns `Option`; counting wants a default
   (`unwrap_or` or `entry(...).or_insert`). Choose, do not guess.
3. **Choose insert-or-update deliberately.** `entry` for accumulate-in-place,
   `insert` only when overwriting is really intended.
4. **Make the result order deterministic.** A `HashMap` has no order, so any answer a
   learner compares must be sorted first — `most_common` breaks ties alphabetically.
The deck's dictionary: you look up a definition using the word as the key.
In Rust that is `HashMap<K, V>` from `std::collections`.

The rules the deck states for dictionaries hold here exactly — all keys are
one type, all values are one type — and Rust adds one more: the key type must
implement `Hash + Eq`, which is why `String` works and `f64` does not.

Reading a map never panics: `get` returns `Option<&V>`. Writing does insert or
overwrite, which is why `entry(...).or_insert(0)` is the idiom for counting.

Count how often each word appears — the deck's `count_word_instances` problem.

Words are split on whitespace and lowercased, so `The` and `the` count
together; punctuation is left alone, which is the honest limit of a lesson
that has not met a tokenizer yet.

The most common word, and how often it appeared. Ties break alphabetically so
the answer is reproducible.

Look a word up and get `0` instead of `None` — the `unwrap_or` habit.

A tiny phone book built with `insert`, then read back with `get`.

