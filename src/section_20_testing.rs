//! Section 20 — testing, the way the course has been graded all along.
//!
//! Every section so far ended in tests, and no section explained them. The `python_lessons`
//! course ships `problems/` pairs and this repository carries a whole `tdd_*` family in
//! the JavaScript cohorts — so the tooling deserves a lesson of its own rather than being
//! something students absorb by osmosis.
//!
//! The parts:
//!
//! | Piece | Meaning |
//! |---|---|
//! | `#[test]` | marks a function as a test; it takes no arguments and returns nothing |
//! | `assert!(cond)` | fails with no message |
//! | `assert_eq!(a, b)` | fails and shows both sides — always prefer this |
//! | `assert_eq!(a, b, "why")` | the message a future reader needs |
//! | `#[should_panic(expected = "…")]` | the test passes only if it panics with that text |
//! | `#[ignore]` | written but not run by default; run with `cargo test -- --ignored` |
//!
//! **Unit tests** live beside the code they test (`#[cfg(test)] mod tests` inside the
//! module) and can reach private functions. **Integration tests** live in `tests/` and see
//! only the public API — this repository's `tests/curriculum.rs` is one. The split is a
//! design pressure: if something is hard to test from outside, the public API is wrong.
//!
//! The loop the course uses: write the test, watch it fail for the right reason, make it
//! pass, then keep it. A test that has never failed has never been proven.

/// A tiny function to test, so the lesson has something honest to operate on.
pub fn divide(dividend: f64, divisor: f64) -> Option<f64> {
    if divisor == 0.0 {
        None
    } else {
        Some(dividend / divisor)
    }
}

/// Is `text` the same forwards and backwards, ignoring case and non-letters?
pub fn is_palindrome(text: &str) -> bool {
    let letters: Vec<char> = text
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    letters.iter().eq(letters.iter().rev())
}

/// The largest number of steps any of `candidates` needs to reach 1 — used by the lesson
/// to show a test that would take too long to run casually.
pub fn longest_collatz(candidates: impl IntoIterator<Item = u64>) -> (u64, u32) {
    let mut best = (1, 0);
    for n in candidates {
        let steps = crate::section_10_loops::collatz_steps(n);
        if steps > best.1 {
            best = (n, steps);
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_test_is_just_a_function_with_an_attribute() {
        assert!(true);
    }

    #[test]
    fn assert_eq_shows_both_sides_when_it_fails() {
        // Change the expected value to 2.0 and run `cargo test` to see the report.
        assert_eq!(divide(10.0, 5.0), Some(2.0));
    }

    #[test]
    fn edge_cases_get_their_own_tests() {
        assert_eq!(divide(1.0, 0.0), None, "division by zero is None, not infinity");
        assert_eq!(divide(0.0, 5.0), Some(0.0));
        assert_eq!(divide(-9.0, 3.0), Some(-3.0));
    }

    #[test]
    fn palindrome_ignores_case_and_punctuation() {
        assert!(is_palindrome("A man, a plan, a canal: Panama"));
        assert!(is_palindrome("racecar"));
        assert!(is_palindrome(""), "the empty string reads the same both ways");
        assert!(!is_palindrome("rust"));
    }

    #[test]
    #[should_panic(expected = "this test exists to show a panic passing")]
    fn should_panic_turns_a_crash_into_a_pass() {
        panic!("this test exists to show a panic passing");
    }

    #[test]
    #[ignore = "takes a second; run with: cargo test -- --ignored"]
    fn the_slow_search_over_a_hundred_thousand_candidates() {
        let (n, steps) = longest_collatz(1..100_000);
        assert!(steps > 300, "the longest chain under 100k is well over 300 steps");
        assert!(n >= 1);
    }

    #[test]
    fn a_filtered_run_names_one_test() {
        // `cargo test palindrome` runs only the tests whose name contains it.
        let names = ["palindrome_ignores_case", "edge_cases", "other"];
        let matched: Vec<&&str> = names.iter().filter(|n| n.contains("palindrome")).collect();
        assert_eq!(matched.len(), 1);
    }
}
