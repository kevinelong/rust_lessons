//! Section 7 — vectors: the list, in Rust.
//!
//! # Subgoals
//!
//! Solving anything with a vector is the same four moves. Name them before writing code —
//! the labels are what the evidence says transfers (see `docs/research/2026-11-06.md`).
//!
//! 1. **Decide the element type.** `Vec<i32>` and `Vec<String>` fail differently; the type
//!    is the first decision, not an afterthought.
//! 2. **Choose the shape of the answer.** `min`/`max` have no answer for an empty list, so
//!    they return `Option`; `sum` does, so it returns a number. Decide this before looping.
//! 3. **Fold over the elements.** One traversal, one accumulator (or one running extreme).
//! 4. **Return without surprising the caller.** Do not mutate an input you were loaned;
//!    `sorted_ascending` copies before it sorts for exactly this reason.
//!
//! # Worked example — the spread of a list of scores
//!
//! The question is "how far apart are the scores?", and the four moves answer it
//! in the order they are named.
//!
//! 1. **Decide the element type.** A score is a whole number, so the input is
//!    `&[i32]` — borrowed, because the caller keeps its list and this function
//!    only reads it.
//! 2. **Choose the shape of the answer.** A list with no scores in it has no
//!    spread, so the answer is `Option<i32>`: `Some(6)` or `None`, never a `0`
//!    that reads as a real gap between two equal scores.
//! 3. **Fold over the elements.** One traversal carries both extremes at once,
//!    which is the whole of the work:
//!
//! ```text
//! let (lo, hi) = values.iter().fold((i32::MAX, i32::MIN), |(lo, hi), &v| (lo.min(v), hi.max(v)));
//! ```
//!
//! 4. **Return without surprising the caller.** The empty list is answered
//!    before the subtraction, because `i32::MAX - i32::MIN` overflows and a debug
//!    build panics on it. The caller's vector is never touched:
//!
//! ```text
//! fn spread(values: &[i32]) -> Option<i32> {
//!     if values.is_empty() {
//!         return None;
//!     }
//!     let (lo, hi) = values.iter().fold((i32::MAX, i32::MIN), |(lo, hi), &v| (lo.min(v), hi.max(v)));
//!     Some(hi - lo)
//! }
//! println!("{:?} {:?}", spread(&[3, 9, 4]), spread(&[]));
//! // Output: Some(6) None
//! ```
//!
//! # From the deck's list to Rust's `Vec`
//!
//! A list is a sequence of values, written `[v1, v2, v3]` in the deck's Python.
//! In Rust a `Vec<T>` is the growable version of that (`[T; N]` is a fixed-size
//! array, and slices `&[T]` are borrowed views of either).
//!
//! Two rules the compiler enforces that the deck has to say out loud:
//!
//! * **Everything in a vector is the same type.** `vec![1, "two"]` does not
//!   compile; use an enum when the values differ in kind.
//! * **Indexing past the end panics**, so prefer `.get(i)` when the index comes
//!   from outside your own code — it returns `Option<&T>` instead of crashing.

/// The smallest value, or `None` for an empty list.
///
/// Note the parameter: `&[i32]` accepts a `Vec<i32>` (by borrowing) *and* an
/// array, because both coerce to a slice. That is why library functions here take
/// slices rather than `Vec`.
pub fn min_of(values: &[i32]) -> Option<i32> {
    values.iter().copied().min()
}

/// The largest value, or `None` for an empty list.
pub fn max_of(values: &[i32]) -> Option<i32> {
    values.iter().copied().max()
}

/// The total, which for an empty list is 0 rather than an error.
pub fn sum_of(values: &[i32]) -> i32 {
    values.iter().sum()
}

/// Sort ascending, returning a new vector — `sort` mutates, so we take a copy
/// first rather than surprising the caller.
pub fn sorted_ascending(values: &[i32]) -> Vec<i32> {
    let mut out = values.to_vec();
    out.sort_unstable();
    out
}

/// Index safely: the position of the first occurrence, if there is one.
pub fn position_of(values: &[i32], needle: i32) -> Option<usize> {
    values.iter().position(|v| *v == needle)
}

/// Build a vector with `push`, the deck's `append` — and show why `Vec` is
/// declared `mut` for that to be allowed.
pub fn range_as_vector(n: u32) -> Vec<u32> {
    let mut out = Vec::new();
    for i in 1..=n {
        out.push(i);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_max_sum_on_a_normal_list() {
        let v = [3, 2, 1, 2];
        assert_eq!(min_of(&v), Some(1));
        assert_eq!(max_of(&v), Some(3));
        assert_eq!(sum_of(&v), 8);
    }

    #[test]
    fn empty_lists_have_no_minimum_but_do_have_a_total() {
        assert_eq!(min_of(&[]), None);
        assert_eq!(max_of(&[]), None);
        assert_eq!(sum_of(&[]), 0);
    }

    #[test]
    fn sorting_does_not_disturb_the_input() {
        let v = vec![3, 1, 2];
        assert_eq!(sorted_ascending(&v), vec![1, 2, 3]);
        assert_eq!(v, vec![3, 1, 2]);
    }

    #[test]
    fn indexing_returns_options_not_panics() {
        assert_eq!(position_of(&[10, 20, 30], 20), Some(1));
        assert_eq!(position_of(&[10, 20, 30], 99), None);
        assert_eq!(range_as_vector(3), vec![1, 2, 3]);
    }
}
