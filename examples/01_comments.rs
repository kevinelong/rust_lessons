//! Section 1, running: names and comments.
//!
//! `cargo run --example 01_comments`

use rust_lessons::section_01_comments::{comment_styles, greeting, is_valid_identifier};

fn main() {
    println!("rust_lessons {} — section 1", rust_lessons::VERSION);

    for (i, style) in comment_styles().iter().enumerate() {
        println!("  {} {style}", i + 1);
    }

    for name in ["total", "total_price", "2price", "total price", "let"] {
        println!("  {name:>12} valid? {}", is_valid_identifier(name));
    }

    println!("{}", greeting("Ada"));
    println!("{}", greeting("   "));
}
