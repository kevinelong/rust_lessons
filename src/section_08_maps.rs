//! Section 8 — maps: looking a value up by another value.
//!
//! # Subgoals
//!
//! 1. **Name the key type.** The key must be `Hash + Eq`; that is why `String` works and
//!    `f64` does not, and it is a decision made before any code.
//! 2. **Decide what a missing key means.** `get` returns `Option`; counting wants a default
//!    (`unwrap_or` or `entry(...).or_insert`). Choose, do not guess.
//! 3. **Choose insert-or-update deliberately.** `entry` for accumulate-in-place,
//!    `insert` only when overwriting is really intended.
//! 4. **Make the result order deterministic.** A `HashMap` has no order, so any answer a
//!    learner compares must be sorted first — `most_common` breaks ties alphabetically.
//!
//! # Worked example — the most common word and its count
//!
//! The question is the deck's `count_word_instances` followed by one lookup, and
//! each move is visible in a line of the answer.
//!
//! 1. **Name the key type.** The key is a word, so it is a `String` — lowercased,
//!    which is a decision made before any code: `String` is `Hash + Eq`, and `f64`
//!    would not be usable as this key at all.
//! 2. **Decide what a missing key means.** Absence is ordinary when counting, so
//!    `count_of` answers `0` for a word nobody wrote, rather than handing back an
//!    `Option` that a caller wanting a number would have to unwrap.
//! 3. **Choose insert-or-update deliberately.** Counting accumulates, so the loop
//!    uses `entry(word).or_insert(0) += 1`. Writing `insert(word, 1)` instead
//!    resets `"the"` to `1` on its third appearance: a silent wrong total, not a
//!    compile error.
//! 4. **Make the result order deterministic.** `most_common` compares the counts
//!    and then the words, so a tie is settled by the alphabet rather than by the
//!    order the iterator happened to produce:
//!
//! ```text
//! let counts = word_counts("The cat and the hat, the end.");
//! println!("{:?}", most_common(&counts));            // Output: Some(("the", 3))
//! println!("{}", count_of(&counts, "dog"));          // Output: 0
//! println!("{:?}", most_common(&word_counts("b a"))); // Output: Some(("a", 1))
//! ```
//!
//! # From the deck's dictionary to `HashMap`
//!
//! The deck's dictionary: you look up a definition using the word as the key.
//! In Rust that is `HashMap<K, V>` from `std::collections`.
//!
//! The rules the deck states for dictionaries hold here exactly — all keys are
//! one type, all values are one type — and Rust adds one more: the key type must
//! implement `Hash + Eq`, which is why `String` works and `f64` does not.
//!
//! Reading a map never panics: `get` returns `Option<&V>`. Writing does insert or
//! overwrite, which is why `entry(...).or_insert(0)` is the idiom for counting.

use std::collections::HashMap;

/// Count how often each word appears — the deck's `count_word_instances` problem.
///
/// Words are split on whitespace and lowercased, so `The` and `the` count
/// together; punctuation is left alone, which is the honest limit of a lesson
/// that has not met a tokenizer yet.
pub fn word_counts(text: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        let key = word.trim_matches(|c: char| !c.is_alphanumeric()).to_lowercase();
        if key.is_empty() {
            continue;
        }
        *counts.entry(key).or_insert(0) += 1;
    }
    counts
}

/// The most common word, and how often it appeared. Ties break alphabetically so
/// the answer is reproducible.
pub fn most_common(counts: &HashMap<String, usize>) -> Option<(String, usize)> {
    counts
        .iter()
        .max_by(|a, b| a.1.cmp(b.1).then(b.0.cmp(a.0)))
        .map(|(word, n)| (word.clone(), *n))
}

/// Look a word up and get `0` instead of `None` — the `unwrap_or` habit.
pub fn count_of(counts: &HashMap<String, usize>, word: &str) -> usize {
    *counts.get(word).unwrap_or(&0)
}

/// A tiny phone book built with `insert`, then read back with `get`.
pub fn phone_book() -> HashMap<String, String> {
    let mut book = HashMap::new();
    book.insert(String::from("ada"), String::from("555-0100"));
    book.insert(String::from("grace"), String::from("555-0101"));
    book
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_words_case_insensitively() {
        let c = word_counts("The cat and the hat, the end.");
        assert_eq!(count_of(&c, "the"), 3);
        assert_eq!(count_of(&c, "cat"), 1);
        assert_eq!(count_of(&c, "missing"), 0);
    }

    #[test]
    fn most_common_is_reproducible_on_ties() {
        let c = word_counts("b a");
        assert_eq!(most_common(&c), Some((String::from("a"), 1)));
        assert_eq!(most_common(&HashMap::new()), None);
    }

    #[test]
    fn phone_book_round_trips() {
        let book = phone_book();
        assert_eq!(book.get("ada").map(String::as_str), Some("555-0100"));
        assert_eq!(book.get("nobody"), None);
        assert_eq!(book.len(), 2);
    }
}
