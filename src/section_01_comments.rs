//! Section 1 — code is written for people.
//!
//! The deck opens with the most important sentence in the course: *the primary
//! audience for source code is not the computer but ourselves and others who
//! will maintain it*. Names and comments are how we write for those readers.
//!
//! Three ideas, and one hard rule for each:
//!
//! * An **identifier** is a name we give a value. In Rust we write names in
//!   `snake_case` — `total_price`, not `TotalPrice` or `total price`.
//! * An identifier may contain letters, digits and underscores, but **may not
//!   start with a digit**, and it may not be a keyword such as `let` or `fn`.
//! * A **comment** is a note for the reader that the compiler ignores: `//` to
//!   the end of the line, `/* ... */` for a block. `///` is a doc comment, and
//!   Rust compiles those into the documentation this course is built from.

/// Keywords a variable may not be named.
const KEYWORDS: &[&str] = &[
    "as", "break", "const", "continue", "crate", "else", "enum", "extern",
    "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod",
    "move", "mut", "pub", "ref", "return", "self", "static", "struct", "super",
    "trait", "true", "type", "unsafe", "use", "where", "while",
];

/// Is `name` a legal Rust identifier under the rules above?
///
/// The rule is narrow on purpose: letters, digits and underscores, first
/// character not a digit, not empty, not a keyword. Comparing identifiers is
/// case sensitive, so `Total` and `total` are two different names.
pub fn is_valid_identifier(name: &str) -> bool {
    let mut chars = name.chars();
    match chars.next() {
        None => return false,
        Some(first) if first.is_alphabetic() || first == '_' => {}
        _ => return false,
    }
    if !chars.all(|c| c.is_alphanumeric() || c == '_') {
        return false;
    }
    !KEYWORDS.contains(&name)
}

/// A greeting built the way the course builds every message: from a name.
///
/// This function exists in every section from here on under different forms,
/// which is the point — the first program you write is still a program.
pub fn greeting(name: &str) -> String {
    // Comments explain *why*, not *what*: the reader can see that this
    // formats a string, but not why the trim is here.
    let trimmed = name.trim();
    if trimmed.is_empty() {
        String::from("Hello, stranger.")
    } else {
        format!("Hello, {trimmed}.")
    }
}

/// The comment styles Rust understands, as data, so the lesson can be tested.
pub fn comment_styles() -> [&'static str; 3] {
    ["// line comment", "/* block comment */", "/// doc comment"]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identifiers_follow_the_rules() {
        assert!(is_valid_identifier("total"));
        assert!(is_valid_identifier("total_price"));
        assert!(is_valid_identifier("price2"));
        assert!(is_valid_identifier("_scratch"));

        assert!(!is_valid_identifier(""));
        assert!(!is_valid_identifier("2price"));
        assert!(!is_valid_identifier("total price"));
        assert!(!is_valid_identifier("total-price"));
        assert!(!is_valid_identifier("let"));
    }

    #[test]
    fn greeting_handles_the_empty_case() {
        assert_eq!(greeting("Ada"), "Hello, Ada.");
        assert_eq!(greeting("  Ada  "), "Hello, Ada.");
        assert_eq!(greeting("   "), "Hello, stranger.");
    }
}
