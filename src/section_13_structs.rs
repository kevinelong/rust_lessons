//! Section 13 — structs, methods and traits.
//!
//! The deck's Alan Kay slide: objects as autonomous cells that talk to each other.
//! Rust has no classes, and it does the job with two pieces:
//!
//! * a **struct** holds the properties (the deck's `self.name`),
//! * an **`impl` block** holds the methods,
//! * a **trait** is the shared contract — the closest thing to an interface, and
//!   the reason two unrelated types can be used the same way.
//!
//! Rust keeps data and behaviour in separate declarations on purpose: you can see
//! exactly what a type stores before reading any of what it does. Encapsulation is
//! privacy (`pub` vs not), not a keyword on a class.

/// The contract every animal in the lesson satisfies.
pub trait Speak {
    /// What this animal says.
    fn sound(&self) -> &'static str;

    /// The species name, shown in the lesson's output.
    fn species(&self) -> &'static str;

    /// A sentence composed from the two pieces above — a default method, so an
    /// implementor only supplies the parts that differ.
    fn speak(&self) -> String {
        format!("The {} says {}", self.species(), self.sound())
    }
}

/// The deck's `Dog` class: a name, a trick count, and methods.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dog {
    pub name: String,
    tricks: u32,
}

impl Dog {
    /// A new dog that knows no tricks yet.
    pub fn new(name: &str) -> Self {
        Dog {
            name: name.to_string(),
            tricks: 0,
        }
    }

    /// A property read through a method — the deck's "member variable".
    pub fn tricks(&self) -> u32 {
        self.tricks
    }

    /// A method that changes the object, hence `&mut self`.
    pub fn learn(&mut self, trick: &str) -> String {
        self.tricks += 1;
        format!("{} learned {trick}", self.name)
    }
}

impl Speak for Dog {
    fn sound(&self) -> &'static str {
        "woof"
    }
    fn species(&self) -> &'static str {
        "dog"
    }
}

/// A second type, to prove the trait is shared rather than special-cased.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cat {
    pub name: String,
}

impl Speak for Cat {
    fn sound(&self) -> &'static str {
        "meow"
    }
    fn species(&self) -> &'static str {
        "cat"
    }
}

/// Take anything that can speak — dynamic dispatch without a class hierarchy.
pub fn chorus(animals: &[&dyn Speak]) -> Vec<String> {
    animals.iter().map(|a| a.speak()).collect()
}

/// A bank account, the classic property/method example, with the invariant that a
/// balance never goes negative.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Account {
    owner: String,
    cents: i64,
}

impl Account {
    /// An account opens at zero.
    pub fn new(owner: &str) -> Self {
        Account {
            owner: owner.to_string(),
            cents: 0,
        }
    }

    /// The current balance in cents.
    pub fn balance(&self) -> i64 {
        self.cents
    }

    /// Add money; returns the new balance.
    pub fn deposit(&mut self, cents: u32) -> i64 {
        self.cents += cents as i64;
        self.cents
    }

    /// Take money out, refusing to overdraw.
    pub fn withdraw(&mut self, cents: u32) -> Result<i64, String> {
        if (cents as i64) > self.cents {
            return Err(format!(
                "{} has {} cents; cannot withdraw {}",
                self.owner, self.cents, cents
            ));
        }
        self.cents -= cents as i64;
        Ok(self.cents)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn methods_change_state_through_mut_self() {
        let mut d = Dog::new("Rex");
        assert_eq!(d.tricks(), 0);
        assert_eq!(d.learn("sit"), "Rex learned sit");
        assert_eq!(d.tricks(), 1);
    }

    #[test]
    fn a_trait_lets_two_types_be_used_the_same_way() {
        let dog = Dog::new("Rex");
        let cat = Cat {
            name: String::from("Mog"),
        };
        let voices = chorus(&[&dog, &cat]);
        assert_eq!(voices, ["The dog says woof", "The cat says meow"]);
    }

    #[test]
    fn the_account_refuses_to_overdraw() {
        let mut a = Account::new("Ada");
        assert_eq!(a.deposit(500), 500);
        assert_eq!(a.withdraw(200), Ok(300));
        assert!(a.withdraw(400).is_err(), "300 < 400 must be refused");
        assert_eq!(a.balance(), 300);
    }
}
