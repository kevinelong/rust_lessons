# 08 — Maps: lookup by key

<sub>rust_lessons 0.20.0 · source: `src/section_08_*.rs` · run: `cargo run --example 08_maps`</sub>

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

# Worked example — the most common word and its count

The question is the deck's `count_word_instances` followed by one lookup, and
each move is visible in a line of the answer.

1. **Name the key type.** The key is a word, so it is a `String` — lowercased,
   which is a decision made before any code: `String` is `Hash + Eq`, and `f64`
   would not be usable as this key at all.
2. **Decide what a missing key means.** Absence is ordinary when counting, so
   `count_of` answers `0` for a word nobody wrote, rather than handing back an
   `Option` that a caller wanting a number would have to unwrap.
3. **Choose insert-or-update deliberately.** Counting accumulates, so the loop
   uses `entry(word).or_insert(0) += 1`. Writing `insert(word, 1)` instead
   resets `"the"` to `1` on its third appearance: a silent wrong total, not a
   compile error.
4. **Make the result order deterministic.** `most_common` compares the counts
   and then the words, so a tie is settled by the alphabet rather than by the
   order the iterator happened to produce:

```text
let counts = word_counts("The cat and the hat, the end.");
println!("{:?}", most_common(&counts));            // Output: Some(("the", 3))
println!("{}", count_of(&counts, "dog"));          // Output: 0
println!("{:?}", most_common(&word_counts("b a"))); // Output: Some(("a", 1))
```

# From the deck's dictionary to `HashMap`

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

