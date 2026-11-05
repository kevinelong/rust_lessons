//! Section 13, running: structs, methods, traits.
//!
//! `cargo run --example 13_structs`

use rust_lessons::section_13_structs::{chorus, Account, Cat, Dog, Speak};

fn main() {
    println!("rust_lessons {} — section 13", rust_lessons::VERSION);

    let mut rex = Dog::new("Rex");
    println!("  {}", rex.learn("sit"));
    println!("  {}", rex.learn("roll over"));
    println!("  {} knows {} tricks", rex.name, rex.tricks());

    let cat = Cat {
        name: String::from("Mog"),
    };
    for line in chorus(&[&rex, &cat]) {
        println!("  {line}");
    }

    // A trait method can be called directly once the trait is in scope.
    println!("  direct trait call: {}", cat.speak());

    println!();
    let mut account = Account::new("Ada");
    println!("  opened at {}", account.balance());
    println!("  deposit 500 -> {}", account.deposit(500));
    println!("  withdraw 200 -> {:?}", account.withdraw(200));
    println!("  withdraw 400 -> {:?}", account.withdraw(400));
    println!("  balance {}", account.balance());
}
