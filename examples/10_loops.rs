//! Section 10, running: for, while, loop.
//!
//! `cargo run --example 10_loops`

use rust_lessons::section_10_loops::{
    collatz_steps, count_vowels, first_index_above, fizzbuzz,
};

fn main() {
    println!("rust_lessons {} — section 10", rust_lessons::VERSION);

    let fizz = fizzbuzz(20);
    println!("  {}", fizz.join(" "));

    println!();
    for n in [1u64, 2, 3, 27] {
        println!("  collatz({n:>2}) reaches 1 in {} steps", collatz_steps(n));
    }

    println!();
    let values = [1, 5, 9];
    println!("  first value above 4 in {values:?} -> {:?}", first_index_above(&values, 4));
    println!("  first value above 9 in {values:?} -> {:?}", first_index_above(&values, 9));
    println!("  vowels in \"hello world\" -> {}", count_vowels("hello world"));
}
