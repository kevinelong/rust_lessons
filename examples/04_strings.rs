//! Section 4, running: text in, text out, and the conversion between text and numbers.
//!
//! `cargo run --example 04_strings`

use rust_lessons::section_04_strings::{
    char_count, parse_sum, shout, twice, with_separators,
};

fn main() {
    println!("rust_lessons {} — section 4", rust_lessons::VERSION);

    println!("  {}", shout(" hello "));
    println!("  {}", twice("na"));
    println!("  {} chars in héllo ({} bytes)", char_count("héllo"), "héllo".len());

    println!();
    for (a, b) in [("2", "3"), (" 2 ", "3"), ("two", "3"), ("-2", "3")] {
        match parse_sum(a, b) {
            Some(total) => println!("  {a:?} + {b:?} = {total}"),
            None => println!("  {a:?} + {b:?} = not numbers"),
        }
    }

    println!();
    for n in [1, 1234, 1234567, -1234] {
        println!("  {n} -> {}", with_separators(n));
    }
}
