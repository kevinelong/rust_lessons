//! Section 7, running: vectors.
//!
//! `cargo run --example 07_vectors`

use rust_lessons::section_07_vectors::{
    max_of, min_of, position_of, range_as_vector, sorted_ascending, sum_of,
};

fn main() {
    println!("rust_lessons {} — section 7", rust_lessons::VERSION);

    let values = vec![3, 2, 1, 2];
    println!("  values    {values:?}");
    println!("  min {:?}  max {:?}  sum {}", min_of(&values), max_of(&values), sum_of(&values));
    println!("  sorted    {:?}", sorted_ascending(&values));
    println!("  original  {values:?}  (unchanged)");
    println!("  index of 2 -> {:?}", position_of(&values, 2));
    println!("  index of 9 -> {:?}", position_of(&values, 9));

    println!();
    let built = range_as_vector(5);
    println!("  built with push: {built:?}");
    println!("  fixed-size array coerces to a slice: min = {:?}", min_of(&[9, 4, 7]));

    println!();
    println!("  empty vec: min = {:?}, sum = {}", min_of(&[]), sum_of(&[]));
}
