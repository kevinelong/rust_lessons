//! Section 12, the interactive version: you type the guesses.
//!
//! `cargo run --example 12_capstone_interactive`
//!
//! This is the deck's "first useful program". Startup reads the clock only to pick
//! a secret, so nothing here needs a dependency; type `q` to give up.

use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};

use rust_lessons::section_12_capstone::Game;

fn main() -> io::Result<()> {
    println!("rust_lessons {} — guess the number", rust_lessons::VERSION);

    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.subsec_nanos() as u32)
        .unwrap_or(0);
    let secret = nanos % 100 + 1;
    let mut game = Game::new(secret, 100);

    println!("I am thinking of a number between 1 and 100. Type q to give up.");

    let stdin = io::stdin();
    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut line = String::new();
        if stdin.read_line(&mut line)? == 0 {
            println!();
            break;
        }
        let line = line.trim();
        if line.eq_ignore_ascii_case("q") {
            println!("The number was {}.", game.secret());
            break;
        }
        let Ok(value) = line.parse::<u32>() else {
            println!("That is not a number between 1 and 100.");
            continue;
        };
        if !(1..=100).contains(&value) {
            println!("Out of range: I said 1 to 100.");
            continue;
        }
        match game.guess(value) {
            Err(attempts) => {
                println!("{} after {attempts} guesses.", Game::hint(std::cmp::Ordering::Equal));
                break;
            }
            Ok(ordering) => println!("{}.", Game::hint(ordering)),
        }
    }
    Ok(())
}
