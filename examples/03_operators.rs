//! Section 3, running: operators, precedence, expressions.
//!
//! `cargo run --example 03_operators`

use rust_lessons::section_03_operators::{
    celsius_to_fahrenheit, grouped_differently, is_even, operator_table, with_parentheses,
    without_parentheses,
};

fn main() {
    println!("rust_lessons {} — section 3", rust_lessons::VERSION);

    for (symbol, example, result) in operator_table() {
        println!("  {symbol:>2}  {example:<10} = {result}");
    }

    println!();
    println!("  a + b * c   (2,3,4) = {}", without_parentheses(2, 3, 4));
    println!("  a + (b * c)         = {}", with_parentheses(2, 3, 4));
    println!("  (a + b) * c         = {}", grouped_differently(2, 3, 4));

    println!();
    for c in [-40.0, 0.0, 37.0, 100.0] {
        println!("  {c:>6.1}C = {:>6.1}F", celsius_to_fahrenheit(c));
    }
    println!("  7 even? {}  8 even? {}", is_even(7), is_even(8));
}
