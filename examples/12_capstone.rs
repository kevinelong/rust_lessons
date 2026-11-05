//! Section 12, running: guess the number, played without a keyboard.
//!
//! `cargo run --example 12_capstone`
//!
//! The interactive version is `examples/12_capstone_interactive.rs`; this one plays
//! a scripted game so the output is reproducible.

use rust_lessons::section_12_capstone::{opening_guess, Game};

fn main() {
    println!("rust_lessons {} — section 12", rust_lessons::VERSION);

    let secret = 42;
    let mut game = Game::new(secret, 100);
    let mut guess = opening_guess(100);

    println!("  secret is between 1 and 100 (the answer is {secret}, for the demo)");
    loop {
        println!("  guessing {guess} ...");
        match game.guess(guess) {
            Err(attempts) => {
                println!("  {} — won in {attempts} attempts", Game::hint(std::cmp::Ordering::Equal));
                break;
            }
            Ok(ordering) => {
                println!("  {}", Game::hint(ordering));
                let (low, high) = game.remaining_range();
                guess = (low + high) / 2;
            }
        }
    }

    println!();
    println!("  A scripted game that never wins:");
    match rust_lessons::section_12_capstone::play(42, &[1, 2, 3]) {
        Ok(attempts) => println!("  won in {attempts}"),
        Err(made) => println!("  gave up after {made} guesses"),
    }
}
