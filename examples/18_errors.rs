//! Section 18, running: the two levels of failure.
//!
//! `cargo run --example 18_errors`

use rust_lessons::section_18_errors::{line_total, order_total, parse_quantity, OrderError};

fn main() {
    println!("rust_lessons {} — section 18", rust_lessons::VERSION);

    for (item, qty) in [("coffee", 2), ("tea", 1), ("caviar", 1), ("coffee", 0)] {
        match line_total(item, qty) {
            Ok(total) => println!("  {item} x{qty} = {total}"),
            Err(e) => println!("  {item} x{qty} = error: {e}"),
        }
    }

    println!();
    match order_total(&[("tea", 2), ("coffee", 1)]) {
        Ok(total) => println!("  order total: {total}"),
        Err(e) => println!("  order failed: {e}"),
    }
    match order_total(&[("tea", 2), ("caviar", 1)]) {
        Ok(total) => println!("  order total: {total}"),
        Err(e) => println!("  order failed: {e}"),
    }

    println!();
    for text in ["3", "three"] {
        match parse_quantity(text) {
            Ok(n) => println!("  parse {text:?} = {n}"),
            Err(e) => println!("  parse {text:?} = error: {e}"),
        }
    }

    println!();
    println!("  unwrap() panics, expect(\"reason\") panics with a reason, ? returns the error.");
    println!("  A library hands back Result: {:?}", OrderError::Unknown(String::from("cake")));
}
