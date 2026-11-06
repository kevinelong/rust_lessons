//! Section 17, running: ownership, moves, and what `&` avoids.
//!
//! `cargo run --example 17_ownership`

use rust_lessons::section_17_ownership::{length_of, take_then_return, Bag};

fn main() {
    println!("rust_lessons {} — section 17", rust_lessons::VERSION);

    let a = 5;
    let b = a; // Copy: both are usable afterwards
    println!("  copy:      a = {a}, b = {b}");

    let owned = String::from("hello");
    let borrowed = &owned;
    println!("  borrow:    length_of({borrowed:?}) = {}", length_of(borrowed));
    println!("  still ours: {owned:?}");

    let cloned = owned.clone();
    println!("  clone:     {cloned:?} is a separate allocation");

    let moved = take_then_return(cloned);
    println!("  move:      returned {moved:?}");

    println!();
    let mut bag = Bag::new("Ada");
    bag.add(String::from("compass"));
    bag.add(String::from("map"));
    println!("  bag owns:  {:?}", bag.list());
    println!("  take(0):   {:?}", bag.take(0));
    println!("  now:       {:?}", bag.list());

    println!();
    println!("  The compiler refuses these, which is the lesson:");
    println!("    let s2 = owned; println!(\"{{}}\", owned);   // use after move");
    println!("    let r = &mut bag; bag.add(..);            // borrow while borrowed");
}
