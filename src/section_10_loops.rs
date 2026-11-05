//! Section 10 — loops.
//!
//! The deck splits loops into two jobs: a **`for`** loop that visits each item of a
//! collection, and a **`while`** loop that repeats until a condition is met. Rust
//! has both plus a third, `loop`, for the case where the exit is somewhere in the
//! middle of the body.
//!
//! Two controls work in all three: `break` leaves the loop early, `continue`
//! skips to the next turn. In Rust `break` can also carry a value out of a `loop`,
//! which is how you write "search until you find it" without a flag variable.

/// FizzBuzz, the exercise every course uses: multiples of 3 say `Fizz`, of 5 say
/// `Buzz`, of both say `FizzBuzz`, everything else is the number as text.
pub fn fizzbuzz(n: u32) -> Vec<String> {
    (1..=n)
        .map(|i| match (i % 3, i % 5) {
            (0, 0) => String::from("FizzBuzz"),
            (0, _) => String::from("Fizz"),
            (_, 0) => String::from("Buzz"),
            _ => i.to_string(),
        })
        .collect()
}

/// How many steps for the Collatz sequence to reach 1 — a `while` loop that must
/// terminate but proves nothing about stopping at 1 for every input.
pub fn collatz_steps(start: u64) -> u32 {
    let mut n = start;
    let mut steps = 0;
    while n != 1 {
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        steps += 1;
    }
    steps
}

/// The index of the first value over the limit, found with `loop` + `break value`.
pub fn first_index_above(values: &[i64], limit: i64) -> Option<usize> {
    let mut i = 0;
    loop {
        if i == values.len() {
            break None;
        }
        if values[i] > limit {
            break Some(i);
        }
        i += 1;
    }
}

/// Count the vowels with a `for` loop and `continue`, to show the skip.
pub fn count_vowels(text: &str) -> usize {
    let mut n = 0;
    for c in text.chars() {
        if !c.is_ascii_alphabetic() {
            continue;
        }
        if "aeiouAEIOU".contains(c) {
            n += 1;
        }
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fizzbuzz_switches_on_both_divisors() {
        let out = fizzbuzz(15);
        assert_eq!(out[0], "1");
        assert_eq!(out[2], "Fizz");
        assert_eq!(out[4], "Buzz");
        assert_eq!(out[14], "FizzBuzz");
        assert_eq!(out.len(), 15);
    }

    #[test]
    fn collatz_counts_reach_one() {
        assert_eq!(collatz_steps(1), 0);
        assert_eq!(collatz_steps(2), 1);
        assert_eq!(collatz_steps(3), 7);
    }

    #[test]
    fn loop_can_return_a_value() {
        assert_eq!(first_index_above(&[1, 5, 9], 4), Some(1));
        assert_eq!(first_index_above(&[1, 2], 4), None);
    }

    #[test]
    fn continue_skips_non_letters() {
        assert_eq!(count_vowels("hello world"), 3);
        assert_eq!(count_vowels("1234"), 0);
    }
}
