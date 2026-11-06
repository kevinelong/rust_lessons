//! Section 21, running: the five errors, their misreadings, and the fixes that compile.
//!
//! `cargo run --example 21_compiler_errors`
//!
//! The broken programs are not in this file: a crate that does not compile cannot teach.
//! Every transcript was captured from `rustc 1.96.0` and lives verbatim in
//! `docs/compiler-errors.md`; here each error is summarised, then its *corrected* form is
//! called so the fix is a running fact rather than a claim.

use rust_lessons::section_21_compiler_errors::{
    append_word, greet, greet_with_default, greeting_twice, parse_count, price,
};

fn main() {
    println!("rust_lessons {} — section 21", rust_lessons::VERSION);

    let cases: [(&str, &str, &str); 5] = [
        (
            "E0308",
            "mismatched types: `let count: i32 = \"three\"`",
            "the annotation promised a number; convert, do not delete the annotation",
        ),
        (
            "E0382",
            "use of moved value: `let owned = name;` then `name`",
            "String is not Copy; borrow to read, clone only for a real second owner",
        ),
        (
            "E0499",
            "two mutable borrows: `&mut text` twice",
            "one &mut at a time; use one borrow twice",
        ),
        (
            "E0004",
            "non-exhaustive patterns: a match missing Strawberry",
            "complete the arm, never a catch-all that prices a future variant at zero",
        ),
        (
            "E0061",
            "wrong number of arguments: `greet(\"Ada\")`",
            "supply both, or add a named single-argument wrapper",
        ),
    ];

    for (code, symptom, fix) in cases {
        println!();
        println!("  {code}  {symptom}");
        println!("        fix: {fix}");
    }

    println!();
    println!("  the fixes, running:");
    println!("    parse_count(\" 42 \")   = {:?}", parse_count(" 42 "));
    println!("    parse_count(\"three\")  is_err = {}", parse_count("three").is_err());

    let (owned, cloned) = greeting_twice(String::from("Ada"));
    println!("    greeting_twice        = {owned:?}, {cloned:?}");

    let mut text = String::from("hi ");
    let len = append_word(&mut text, "there");
    println!("    append_word           = {text:?} ({len} bytes)");

    println!("    price(\"strawberry\")   = {:?}", price("strawberry"));
    println!("    greet(\"Ada\", \"Hello\") = {:?}", greet("Ada", "Hello"));
    println!("    greet_with_default    = {:?}", greet_with_default("Ada"));

    println!();
    println!("  Full transcripts: docs/compiler-errors.md (captured, not remembered).");
}
