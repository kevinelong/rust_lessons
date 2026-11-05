//! Section 5 — booleans and conditionals.
//!
//! A `bool` is `true` or `false`, and is usually *produced* by comparing two
//! values rather than typed by hand: `==`, `!=`, `<`, `<=`, `>`, `>=`.
//!
//! The deck's rule of thumb is worth restating in Rust because the compiler
//! enforces it: **the condition of an `if` must be a `bool`**. There is no
//! truthiness — `if count { }` is an error, and `if count > 0 { }` is the fix.
//!
//! Rust's `if` is an expression: it produces a value, so both branches must have
//! the same type.

/// Grade a score the way the deck's exercise does, with an `if`/`else` chain.
pub fn classify(score: u32) -> &'static str {
    if score >= 90 {
        "A"
    } else if score >= 80 {
        "B"
    } else if score >= 70 {
        "C"
    } else if score >= 60 {
        "D"
    } else {
        "F"
    }
}

/// The sign of a number, using `if` as an expression.
pub fn sign(n: i64) -> &'static str {
    if n > 0 {
        "positive"
    } else if n < 0 {
        "negative"
    } else {
        "zero"
    }
}

/// Boolean operators: `&&` and, `||` or, `!` not. Note that `&&` short-circuits,
/// so the right-hand side is not evaluated when the left already decides it.
pub fn can_drive(age: u32, has_licence: bool) -> bool {
    age >= 16 && has_licence
}

/// The comparison operators, as (symbol, meaning).
pub fn comparison_table() -> [(&'static str, &'static str); 6] {
    [
        ("==", "equal to"),
        ("!=", "not equal to"),
        ("<", "less than"),
        ("<=", "less than or equal to"),
        (">", "greater than"),
        (">=", "greater than or equal to"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grade_boundaries_are_inclusive() {
        assert_eq!(classify(100), "A");
        assert_eq!(classify(90), "A");
        assert_eq!(classify(89), "B");
        assert_eq!(classify(80), "B");
        assert_eq!(classify(70), "C");
        assert_eq!(classify(60), "D");
        assert_eq!(classify(59), "F");
        assert_eq!(classify(0), "F");
    }

    #[test]
    fn sign_covers_all_three_cases() {
        assert_eq!(sign(5), "positive");
        assert_eq!(sign(-5), "negative");
        assert_eq!(sign(0), "zero");
    }

    #[test]
    fn and_requires_both_sides() {
        assert!(can_drive(16, true));
        assert!(!can_drive(16, false));
        assert!(!can_drive(15, true));
    }
}
