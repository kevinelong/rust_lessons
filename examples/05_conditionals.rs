//! Section 5, running: booleans, comparison, branching.
//!
//! `cargo run --example 05_conditionals`

use rust_lessons::section_05_conditionals::{
    can_drive, classify, comparison_table, sign,
};

fn main() {
    println!("rust_lessons {} — section 5", rust_lessons::VERSION);

    for (symbol, meaning) in comparison_table() {
        println!("  {symbol:>2}  {meaning}");
    }

    println!();
    for score in [95, 90, 85, 72, 61, 59] {
        println!("  {score:>3} -> {}", classify(score));
    }

    println!();
    for n in [5, 0, -5] {
        println!("  {n:>3} is {}", sign(n));
    }

    println!();
    println!("  age 16 with licence: {}", can_drive(16, true));
    println!("  age 16 without:      {}", can_drive(16, false));

    // `if` is an expression: this is legal Rust and has the same type in both arms.
    let label = if can_drive(15, true) { "go" } else { "wait" };
    println!("  as an expression:    {label}");
}
