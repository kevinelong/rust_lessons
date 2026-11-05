//! Section 2, running: variables, mutability, types and shadowing.
//!
//! `cargo run --example 02_variables`

use rust_lessons::section_02_variables::{
    average, average_without_casting, describe_types, shadowing_demo, INTEGER_TYPES,
};

fn main() {
    println!("rust_lessons {} — section 2", rust_lessons::VERSION);

    let immutable = 5;
    let mut mutable = 5;
    mutable += 1;
    println!("  immutable = {immutable}, mutable = {mutable}");

    // Shadowing is a new binding, not a reassignment, so the type may change.
    let value = "41";
    let value: i64 = value.parse().unwrap();
    println!("  shadowed text -> number: {value} (demo says {})", shadowing_demo());

    for (name, bits) in INTEGER_TYPES {
        println!("  {name} is {bits} bits");
    }
    for (name, note) in describe_types() {
        println!("  {name}: {note}");
    }

    println!("  average(1, 2)              = {}", average(1, 2));
    println!("  average_without_casting(1, 2) = {}  <- integer division", average_without_casting(1, 2));
}
