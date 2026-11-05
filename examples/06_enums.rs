//! Section 6, running: enums as your own types.
//!
//! `cargo run --example 06_enums`

use rust_lessons::section_06_enums::{half_of, Payment, Shake};

fn main() {
    println!("rust_lessons {} — section 6", rust_lessons::VERSION);

    let mut total = 0;
    for shake in Shake::all() {
        println!("  {:>10} {:>4} cents", shake.label(), shake.price_cents());
        total += shake.price_cents();
    }
    println!("  one of each: {total} cents");

    println!();
    for payment in [
        Payment::Cash(500),
        Payment::Card { last4: 4242, cents: 250 },
        Payment::Refunded("duplicate charge"),
    ] {
        println!("  {payment:?} -> net {} cents", payment.net_cents());
    }

    println!();
    for n in [10, 7] {
        match half_of(n) {
            Some(h) => println!("  half of {n} is {h}"),
            None => println!("  {n} has no whole half"),
        }
    }
}
