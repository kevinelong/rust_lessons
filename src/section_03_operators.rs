//! Section 3 — operators and expressions.
//!
//! The deck introduces `+ - * /` and parentheses, and calls the result of
//! combining variables and operators an **expression**. Rust adds two things
//! worth teaching straight away:
//!
//! * `%` (remainder) and `&& / || / !` for the boolean operators.
//! * **Order of operations is not negotiable**: `2 + 3 * 4` is 14. When the
//!   intent is not obvious from reading it, parenthesise it — the compiler will
//!   not guess what you meant.

/// `a + b * c`, without parentheses, so the lesson can show what precedence does.
pub fn without_parentheses(a: i64, b: i64, c: i64) -> i64 {
    a + b * c
}

/// The same arithmetic with the order written down.
pub fn with_parentheses(a: i64, b: i64, c: i64) -> i64 {
    a + (b * c)
}

/// A different order, a different answer: `(a + b) * c`.
pub fn grouped_differently(a: i64, b: i64, c: i64) -> i64 {
    (a + b) * c
}

/// Celsius to Fahrenheit — the classic expression: `(c * 9 / 5) + 32`.
///
/// Order matters here too. Multiplying before dividing keeps the full precision
/// of the intermediate value; dividing first would lose it.
pub fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

/// Even or odd, decided by the remainder operator.
pub fn is_even(n: i64) -> bool {
    n % 2 == 0
}

/// The arithmetic operators the course uses, as (symbol, example, result).
pub fn operator_table() -> [(&'static str, &'static str, &'static str); 5] {
    [
        ("+", "7 + 3", "10"),
        ("-", "7 - 3", "4"),
        ("*", "7 * 3", "21"),
        ("/", "7 / 3", "2 (integer division)"),
        ("%", "7 % 3", "1"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn precedence_matches_arithmetic() {
        assert_eq!(without_parentheses(2, 3, 4), 14);
        assert_eq!(with_parentheses(2, 3, 4), 14);
        assert_eq!(grouped_differently(2, 3, 4), 20);
    }

    #[test]
    fn temperature_conversion_is_exact_at_known_points() {
        assert_eq!(celsius_to_fahrenheit(0.0), 32.0);
        assert_eq!(celsius_to_fahrenheit(100.0), 212.0);
        assert_eq!(celsius_to_fahrenheit(-40.0), -40.0);
    }

    #[test]
    fn remainder_identifies_evens() {
        assert!(is_even(2));
        assert!(is_even(0));
        assert!(!is_even(3));
        assert!(!is_even(-1));
    }
}
