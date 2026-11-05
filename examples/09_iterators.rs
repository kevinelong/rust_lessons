//! Section 9, running: ranges and iterators.
//!
//! `cargo run --example 09_iterators`

use rust_lessons::section_09_iterators::{
    count_above, evens_up_to, first_squares, long_words_uppercased, running_totals, total,
};

fn main() {
    println!("rust_lessons {} — section 9", rust_lessons::VERSION);

    println!("  evens up to 12:     {:?}", evens_up_to(12));
    println!("  first 6 squares:    {:?}", first_squares(6));

    let values = [1, 2, 3, 4];
    println!("  total {values:?}:         {}", total(&values));
    println!("  count above 2:      {}", count_above(&values, 2));
    println!("  running totals:     {:?}", running_totals(&values));

    println!();
    println!("  {:?}", long_words_uppercased("a big red balloon for the win"));

    // Laziness, made visible: the range is infinite, take() is what ends it.
    let first_five: Vec<u64> = (1u64..).take(5).collect();
    println!("  (1..).take(5)       {first_five:?}");
}
