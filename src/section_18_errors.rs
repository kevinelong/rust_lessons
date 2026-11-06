//! Section 18 — errors: `assert`, `panic!`, `Result` and `?`.
//!
//! `python_lessons` teaches this as `assert_try_except.py`; the deck itself has no
//! slide for it. A program that cannot fail gracefully is a program that stops working
//! the first time the world misbehaves, so this belongs in the course regardless.
//!
//! Rust has two levels, and the choice between them is a design decision, not a style
//! preference:
//!
//! | Level | Written | Means | Use for |
//! |---|---|---|---|
//! | unrecoverable | `panic!`, `assert!`, `unwrap()` | this is a bug; stop | broken invariants, tests |
//! | recoverable | `Result<T, E>`, `?` | the caller must decide | missing files, bad input, network |
//!
//! `?` is the sentence-maker: it either yields the value or returns the error to the
//! caller, so a function that may fail reads like the happy path.

//! # Diagram
//!
//! `docs/diagrams/question-mark-propagation.svg` follows an error up three frames and
//! shows what the `?` skipped on the way.
//!

use std::fmt;

/// A domain error, written out — the Rust answer to a `try`/`except ValueError`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrderError {
    /// The quantity was not a positive number.
    NotPositive(i64),
    /// The item is not on the menu.
    Unknown(String),
    /// The total would overflow what we can charge.
    TooLarge(u64),
}

impl fmt::Display for OrderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderError::NotPositive(n) => write!(f, "quantity must be positive, got {n}"),
            OrderError::Unknown(item) => write!(f, "no such item: {item}"),
            OrderError::TooLarge(total) => write!(f, "total {total} is too large to charge"),
        }
    }
}

impl std::error::Error for OrderError {}

/// The prices, in the smallest unit, as the only place prices are written.
fn price_of(item: &str) -> Option<u64> {
    match item.to_lowercase().as_str() {
        "coffee" => Some(350),
        "tea" => Some(275),
        "sandwich" => Some(750),
        _ => None,
    }
}

/// Price `quantity` of `item`, refusing the three things that can be wrong.
///
/// Written with `?` throughout: every check is one line and the happy path is the last
/// expression, which is what `?` is for.
pub fn line_total(item: &str, quantity: i64) -> Result<u64, OrderError> {
    if quantity <= 0 {
        return Err(OrderError::NotPositive(quantity));
    }
    let unit = price_of(item).ok_or_else(|| OrderError::Unknown(item.to_string()))?;
    let total = unit
        .checked_mul(quantity as u64)
        .ok_or(OrderError::TooLarge(u64::MAX))?;
    if total > 100_000_000 {
        return Err(OrderError::TooLarge(total));
    }
    Ok(total)
}

/// Add up a whole order, stopping at the first bad line.
///
/// `collect::<Result<Vec<_>, _>>()` is the idiom for "all or nothing": it returns the
/// first error rather than a partially built list.
pub fn order_total(lines: &[(&str, i64)]) -> Result<u64, OrderError> {
    let totals: Result<Vec<u64>, OrderError> =
        lines.iter().map(|(item, qty)| line_total(item, *qty)).collect();
    Ok(totals?.iter().sum())
}

/// The `expect` habit, in the one place it is appropriate: a value we control.
///
/// `unwrap()` panics with no context; `expect("why")` panics with the reason. In
/// library code neither belongs — the caller gets a `Result` instead.
pub fn parse_quantity(text: &str) -> Result<i64, OrderError> {
    text.trim()
        .parse::<i64>()
        .map_err(|_| OrderError::NotPositive(0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path_is_a_single_expression() {
        assert_eq!(line_total("coffee", 2), Ok(700));
        assert_eq!(line_total("Tea", 1), Ok(275));
    }

    #[test]
    fn each_failure_has_its_own_error() {
        assert_eq!(line_total("coffee", 0), Err(OrderError::NotPositive(0)));
        assert_eq!(line_total("coffee", -3), Err(OrderError::NotPositive(-3)));
        assert_eq!(
            line_total("caviar", 1),
            Err(OrderError::Unknown(String::from("caviar")))
        );
        assert_eq!(line_total("sandwich", 200_000), Err(OrderError::TooLarge(150_000_000)));
    }

    #[test]
    fn display_says_what_went_wrong() {
        assert_eq!(
            OrderError::Unknown(String::from("caviar")).to_string(),
            "no such item: caviar"
        );
        assert_eq!(OrderError::NotPositive(-3).to_string(), "quantity must be positive, got -3");
    }

    #[test]
    fn a_bad_line_fails_the_whole_order() {
        assert_eq!(order_total(&[("tea", 2), ("coffee", 1)]), Ok(900), "275*2 + 350");
        assert!(order_total(&[("tea", 2), ("caviar", 1)]).is_err());
        assert_eq!(order_total(&[]), Ok(0));
    }

    #[test]
    fn parsing_reports_rather_than_panics() {
        assert_eq!(parse_quantity(" 3 "), Ok(3));
        assert!(parse_quantity("three").is_err());
    }

    #[test]
    #[should_panic(expected = "the price list has exactly three entries")]
    fn an_assertion_carries_its_reason() {
        // The invariant: `price_of` knows exactly these three items. If someone adds a
        // fourth price and forgets this list, the assertion says why it stopped.
        let known = ["coffee", "tea", "sandwich"];
        assert_eq!(known.len(), 3, "the price list has exactly three entries");
        let _also_known = ["coffee", "tea", "sandwich", "cake"]; // the fourth one
        assert_eq!(_also_known.len(), 3, "the price list has exactly three entries");
    }
}
