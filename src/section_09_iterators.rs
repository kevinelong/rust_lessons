//! Section 9 — ranges and iterators.
//!
//! The deck's "ranges may contain an infinite number of items; they save memory by
//! generating each item as we iterate over them" is exactly what a Rust iterator
//! is. `1..=1000` does not build a list of a thousand numbers; it is a recipe for
//! producing them one at a time.
//!
//! An iterator is lazy: `map` and `filter` describe work and do none of it until
//! something consumes the chain — `collect`, `sum`, `count`, `for`. That laziness
//! is why `(1..).map(...).take(5)` terminates and `(1..).collect::<Vec<_>>()` does
//! not.

//! # Diagram
//!
//! `docs/diagrams/iterator-pipeline.svg`: lazy stages on the left, the consuming stage
//! that actually runs them on the right.
//!

/// The even numbers from 2 up to `n`, inclusive.
pub fn evens_up_to(n: u32) -> Vec<u32> {
    (1..=n).filter(|x| x % 2 == 0).collect()
}

/// Total a slice with an iterator rather than a loop.
pub fn total(values: &[i64]) -> i64 {
    values.iter().sum()
}

/// How many values exceed `limit` — `filter` then `count`.
pub fn count_above(values: &[i64], limit: i64) -> usize {
    values.iter().filter(|v| **v > limit).count()
}

/// The first `n` squares, from an infinite range: laziness, made visible.
pub fn first_squares(n: usize) -> Vec<u64> {
    (1u64..).map(|x| x * x).take(n).collect()
}

/// Running totals — `scan` folds and keeps each intermediate value.
pub fn running_totals(values: &[i64]) -> Vec<i64> {
    values
        .iter()
        .scan(0i64, |acc, v| {
            *acc += v;
            Some(*acc)
        })
        .collect()
}

/// Words longer than three characters, uppercased — a chain that reads like the
/// English description of the job.
pub fn long_words_uppercased(text: &str) -> Vec<String> {
    text.split_whitespace()
        .filter(|w| w.chars().count() > 3)
        .map(|w| w.to_uppercase())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evens_are_inclusive_of_the_bound() {
        assert_eq!(evens_up_to(6), vec![2, 4, 6]);
        assert_eq!(evens_up_to(1), Vec::<u32>::new());
    }

    #[test]
    fn iterator_aggregates_match_the_loops_they_replace() {
        let v = [1, 2, 3, 4];
        assert_eq!(total(&v), 10);
        assert_eq!(count_above(&v, 2), 2);
        assert_eq!(running_totals(&v), vec![1, 3, 6, 10]);
    }

    #[test]
    fn infinite_ranges_are_safe_when_taken_from() {
        assert_eq!(first_squares(4), vec![1, 4, 9, 16]);
    }

    #[test]
    fn chains_read_like_the_description() {
        assert_eq!(
            long_words_uppercased("a big red balloon"),
            vec!["BALLOON".to_string()]
        );
    }
}
