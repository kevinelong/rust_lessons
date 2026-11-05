//! Section 2 — variables and data types.
//!
//! A variable is a labelled container for a value. Rust makes two things
//! explicit that Python and JavaScript leave implicit, and both are lessons the
//! deck already teaches:
//!
//! * A variable is **immutable unless you say `mut`**. `let x = 5;` cannot be
//!   reassigned; `let mut x = 5;` can. You are told when your design assumed
//!   something would change.
//! * A variable **always has a type**, and the compiler infers it from the value
//!   when you do not write it: `let count = 3` is an `i32`.
//!
//! `let` again with the same name is **shadowing**, not reassignment — the
//! second binding is a new variable that may even have a different type. That is
//! how you convert a value in place: `let text = "42"; let text = text.parse()?;`

/// The integer types, as (name, bits) — the deck's "integers are whole numbers".
pub const INTEGER_TYPES: &[(&str, u32)] = &[
    ("i8", 8), ("i16", 16), ("i32", 32), ("i64", 64), ("i128", 128), ("isize", 64),
];

/// The other basic types the deck introduces, by the name the deck uses.
pub fn describe_types() -> [(&'static str, &'static str); 4] {
    [
        ("int", "i32 by default — whole numbers such as 1, 2, 0, -1"),
        ("float", "f64 by default — fractional numbers such as 0.5"),
        ("bool", "true or false"),
        ("str", "&str is borrowed text, String is owned text"),
    ]
}

/// The average of two integers — and the trap this lesson exists to show.
///
/// `(a + b) / 2` with two `i64` values performs **integer division**: the
/// fractional part is discarded, and `(1 + 2) / 2` is `1`, not `1.5`. Casting
/// both sides to `f64` first is the fix, and the reason the declaration says
/// `-> f64` while the inputs are integers.
pub fn average(a: i64, b: i64) -> f64 {
    (a as f64 + b as f64) / 2.0
}

/// The same average done the wrong way, kept only so tests can prove it differs.
pub fn average_without_casting(a: i64, b: i64) -> f64 {
    ((a + b) / 2) as f64
}

/// A value converted by shadowing: text in, number out, same name.
pub fn shadowing_demo() -> i64 {
    let value = "41";
    let value: i64 = value.parse().unwrap();
    value + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn average_converts_before_dividing() {
        assert_eq!(average(1, 2), 1.5);
        assert_eq!(average(2, 2), 2.0);
        assert_eq!(average_without_casting(1, 2), 1.0, "integer division truncates");
    }

    #[test]
    fn shadowing_changes_the_type() {
        assert_eq!(shadowing_demo(), 42);
    }

    #[test]
    fn types_cover_the_deck_list() {
        let names: Vec<&str> = describe_types().iter().map(|(n, _)| *n).collect();
        assert_eq!(names, ["int", "float", "bool", "str"]);
    }
}
