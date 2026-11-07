//! Section 4 — strings, numbers, and converting between them.
//!
//! The deck's most-used data type is text, and its most common operation is
//! *conversion*: a number turned into text to print, text turned into a number to
//! calculate with.
//!
//! Rust splits what other languages call "a string" in two, and it is worth the
//! trouble to learn early:
//!
//! * `&str` is **borrowed** text — a view into characters someone else owns. For a string
//!   literal such as `"hi"`, the owner is the compiled binary: the view lives for the whole
//!   program (`&'static str`), which is why literals need no lifetime of their own. What
//!   "borrowed" *means* — and how a borrow differs from a move — is section 17; here it is
//!   only a name for "a view, not the text itself".
//! * `String` is **owned** text you can grow. `String::from("x")`, `.push_str()`,
//!   `format!`.
//!

//! # Diagram
//!
//! `docs/diagrams/string-stack-heap.svg` draws the two words a `String` keeps on the
//! stack and the bytes it points at on the heap.
//!
//! Conversion is explicit and can fail, so `parse` returns a `Result`: text that
//! is not a number is an error you must handle, not a silent `NaN`.
/// Uppercase text with an exclamation mark — the deck's `twice()` example's cousin.
pub fn shout(text: &str) -> String {
    format!("{}!", text.trim().to_uppercase())
}

/// Repeat text, the `twice()` exercise from the deck: `&str` in, owned `String` out.
pub fn twice(text: &str) -> String {
    format!("{text}{text}")
}

/// Add two numbers written as text. `None` when either is not a number.
///
/// This is the honest version of "convert a string to a number": the failure is
/// part of the type, so a caller cannot forget it.
pub fn parse_sum(a: &str, b: &str) -> Option<i64> {
    let a: i64 = a.trim().parse().ok()?;
    let b: i64 = b.trim().parse().ok()?;
    Some(a + b)
}

/// A number rendered back into text with a thousands separator, to show that
/// going the other way is lossless.
pub fn with_separators(n: i64) -> String {
    let negative = n < 0;
    let digits = n.unsigned_abs().to_string();
    let mut out = String::new();
    for (i, c) in digits.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            out.push(',');
        }
        out.push(c);
    }
    let grouped: String = out.chars().rev().collect();
    if negative {
        format!("-{grouped}")
    } else {
        grouped
    }
}

/// The number of `char`s — not bytes. `"hi"` is 2, `"héllo"` is 5 while its
/// byte length is 6, which is the difference this lesson points out.
pub fn char_count(text: &str) -> usize {
    text.chars().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shouting_trims_and_uppercases() {
        assert_eq!(shout(" hello "), "HELLO!");
        assert_eq!(twice("ab"), "abab");
    }

    #[test]
    fn parsing_failures_are_values_not_panics() {
        assert_eq!(parse_sum("2", "3"), Some(5));
        assert_eq!(parse_sum(" 2 ", "3"), Some(5));
        assert_eq!(parse_sum("two", "3"), None);
        assert_eq!(parse_sum("-2", "3"), Some(1));
    }

    #[test]
    fn separators_group_by_three() {
        assert_eq!(with_separators(1), "1");
        assert_eq!(with_separators(123), "123");
        assert_eq!(with_separators(1234), "1,234");
        assert_eq!(with_separators(1234567), "1,234,567");
        assert_eq!(with_separators(-1234), "-1,234");
    }

    #[test]
    fn chars_are_not_bytes() {
        assert_eq!(char_count("hi"), 2);
        assert_eq!(char_count("héllo"), 5);
        assert_eq!("héllo".len(), 6, "bytes, not characters");
    }
}
