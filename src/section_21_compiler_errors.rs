//! Section 21 — reading compiler errors.
//!
//! The CMU study of Rust learners puts compiler messages near the top of the barrier list,
//! and the Rust cohorts in the earlier courses never covered it: the Ruby track had a lesson
//! called *reading error messages* and the Python deck had none. So this section exists,
//! and its transcripts are **captured from `rustc 1.96.0`**, not remembered — see
//! `docs/compiler-errors.md` for the five full outputs, verbatim.
//!
//! What an error contains, in the order you should read it:
//!
//! 1. **the code** (`E0308`) — stable and searchable;
//! 2. **the location** — where the compiler *concluded* the problem, which is not always
//!    where the mistake is;
//! 3. **the quoted source line**;
//! 4. **the span** (`^^^^`) — the exact expression under discussion;
//! 5. **the note** — the part beginners skip and the part that usually contains the fix.
//!
//! Each function below is the *corrected* form of one captured error, with the test that
//! proves it works. The broken form is not in this module: a crate that does not compile
//! cannot teach, which is why the broken inputs live in the notes and the fixes live here.

/// **E0308, mismatched types.** `let count: i32 = "three";`
///
/// The annotation promised a number and the value was text. The fix is to convert, not to
/// delete the annotation: `parse` returns a `Result` because `"three"` cannot become a number.
pub fn parse_count(text: &str) -> Result<i32, std::num::ParseIntError> {
    text.trim().parse()
}

/// **E0382, use of moved value.** `let owned = name;` then using `name`.
///
/// `String` is not `Copy`: assigning moved the buffer and the old name is dead. Borrow when
/// you only need to read, clone when you truly need two owners, and say which in a comment —
/// a clone that is not explained is usually a misunderstanding.
pub fn greeting_twice(name: String) -> (String, String) {
    let owned = name; // move: `name` is gone from here on
    let borrowed = owned.clone(); // deliberate second owner
    (owned, borrowed)
}

/// **E0499, two mutable borrows.** `let a = &mut text; let b = &mut text;`
///
/// At most one `&mut` may exist at a time. The fix is to use one borrow twice in sequence,
/// which is exactly what the borrow checker is asking for.
pub fn append_word(text: &mut String, word: &str) -> usize {
    text.push_str(word);
    text.push(' ');
    text.len() // one borrow, used twice — not two borrows alive at once
}

/// **E0004, non-exhaustive patterns.** a `match` missing `Strawberry`.
///
/// The compiler names the missing arm. Completing the enum is the fix; a `_ => 0` arm would
/// compile and would silently price a future variant at zero — which is why the course never
/// uses a catch-all for a domain enum.
pub fn price(shake: &str) -> Option<u32> {
    match shake {
        "vanilla" => Some(350),
        "chocolate" => Some(400),
        "strawberry" => Some(375),
        _ => None, // an unknown name is not a variant: `Option` says so
    }
}

/// **E0061, wrong number of arguments.** `greet("Ada")` against `greet(name, greeting)`.
///
/// The note lists the expected arguments in order. The fix is to supply both — and, if the
/// second is usually the same, to give it a default by taking `Option` or by offering a
/// two-argument wrapper rather than changing every call site.
pub fn greet(name: &str, greeting: &str) -> String {
    format!("{greeting}, {name}")
}

/// The single-argument form `E0061` was asking for, kept as a named convenience.
pub fn greet_with_default(name: &str) -> String {
    greet(name, "Hello")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn e0308_the_annotation_survives_the_fix() {
        assert_eq!(parse_count(" 42 "), Ok(42));
        assert!(parse_count("three").is_err(), "text that is not a number stays an error");
    }

    #[test]
    fn e0382_the_second_owner_is_deliberate() {
        let (owned, cloned) = greeting_twice(String::from("Ada"));
        assert_eq!(owned, "Ada");
        assert_eq!(cloned, "Ada");
        assert_ne!(owned.as_ptr(), cloned.as_ptr(), "two allocations, not one moved twice");
    }

    #[test]
    fn e0499_one_borrow_used_twice() {
        let mut text = String::from("hi ");
        let len = append_word(&mut text, "there");
        assert_eq!(text, "hi there ");
        assert_eq!(len, 9);
    }

    #[test]
    fn e0004_every_variant_has_a_price() {
        assert_eq!(price("vanilla"), Some(350));
        assert_eq!(price("strawberry"), Some(375), "the arm E0004 named");
        assert_eq!(price("custard"), None, "unknown is not a variant");
    }

    #[test]
    fn e0061_both_arguments_or_the_wrapper() {
        assert_eq!(greet("Ada", "Hello"), "Hello, Ada");
        assert_eq!(greet_with_default("Ada"), "Hello, Ada");
    }
}
