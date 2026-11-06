//! Section 20, running: what the tests are, and how to run one at a time.
//!
//! `cargo run --example 20_testing`
//! `cargo test`                    everything
//! `cargo test palindrome`         just the tests whose name matches
//! `cargo test -- --ignored`       also the tests marked #[ignore]
//! `cargo test -- --nocapture`     let the tests print

use rust_lessons::section_20_testing::{divide, is_palindrome, longest_collatz};

fn main() {
    println!("rust_lessons {} — section 20", rust_lessons::VERSION);

    println!();
    println!("  the functions under test, called directly:");
    println!("    divide(10.0, 5.0)   = {:?}", divide(10.0, 5.0));
    println!("    divide(1.0, 0.0)    = {:?}", divide(1.0, 0.0));
    println!("    is_palindrome(..)   = {}", is_palindrome("A man, a plan, a canal: Panama"));

    let (n, steps) = longest_collatz(1..10_000);
    println!("    longest_collatz under 10k = {n} with {steps} steps");

    println!();
    println!("  the attributes in this section's tests:");
    for (attribute, meaning) in [
        ("#[test]", "marks a function as a test"),
        ("assert_eq!(a, b, \"why\")", "fails and prints both sides plus the reason"),
        ("#[should_panic(expected = \"…\")]", "the test passes only if it panics with that text"),
        ("#[ignore = \"reason\"]", "written but not run unless asked"),
    ] {
        println!("    {attribute:<36} {meaning}");
    }

    println!();
    println!("  unit tests sit beside the code; integration tests in tests/ see only the API.");
    println!("  Red, green, then keep: a test that never failed has never been proven.");
}
