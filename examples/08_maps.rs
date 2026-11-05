//! Section 8, running: maps.
//!
//! `cargo run --example 08_maps`

use rust_lessons::section_08_maps::{count_of, most_common, phone_book, word_counts};

fn main() {
    println!("rust_lessons {} — section 8", rust_lessons::VERSION);

    let text = "The cat and the hat, the end of the story.";
    let counts = word_counts(text);
    println!("  {text}");

    let mut pairs: Vec<_> = counts.iter().collect();
    pairs.sort();
    for (word, n) in pairs {
        println!("    {word:>8} {n}");
    }

    println!();
    println!("  most common: {:?}", most_common(&counts));
    println!("  count of 'the': {}", count_of(&counts, "the"));
    println!("  count of 'dog': {}", count_of(&counts, "dog"));

    println!();
    let book = phone_book();
    println!("  phone book: {book:?}");
}
