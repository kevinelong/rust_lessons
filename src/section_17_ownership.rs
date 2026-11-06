//! Section 17 — ownership and moves (Rust requires this; the deck never had to say it).
//!
//! Sections 04, 07 and 13 borrow with `&` before this one explains *why* borrowing
//! exists. That order was a real gap: you cannot understand `&` until you know what it
//! avoids. This section pays that debt.
//!
//! Every value has exactly **one owner**. When the owner goes out of scope the value is
//! freed — no garbage collector, no manual `free()`. Two consequences follow, and both
//! are compile errors rather than bugs:
//!
//! * **Moving**: assigning or passing a value that owns its data (`String`, `Vec`)
//!   transfers ownership. The old name is dead; using it is an error.
//! * **Copying**: small fixed-size types (`i32`, `f64`, `bool`, `char`, a fixed-size
//!   array of them) are `Copy`, so passing them copies and the original stays usable.
//!
//! `&` exists to avoid doing either: borrow instead of move, and keep the value.

//! # Diagrams
//!
//! `docs/diagrams/ownership.svg` draws move, borrow and clone side by side;
//! `docs/diagrams/borrow-rules.svg` draws the count rule that makes them safe.
//!

/// Take ownership of a `String` and give it back, so the lesson can show the hand-over.
pub fn take_then_return(text: String) -> String {
    // Leaving the function alone would drop `text`. Returning it moves it out.
    text
}

/// Borrow a `String` and use it — the caller keeps their value, and this is why the
/// rest of the course writes `&str` in its signatures.
pub fn length_of(text: &str) -> usize {
    text.chars().count()
}

/// A struct that owns its data: moving it moves everything inside it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bag {
    pub owner: String,
    pub items: Vec<String>,
}

impl Bag {
    /// A new bag.
    pub fn new(owner: &str) -> Self {
        Bag {
            owner: owner.to_string(),
            items: Vec::new(),
        }
    }

    /// Put something in, by value — the item is now the bag's.
    pub fn add(&mut self, item: String) {
        self.items.push(item);
    }

    /// Take something out, giving it away — after this the bag no longer has it.
    pub fn take(&mut self, index: usize) -> Option<String> {
        if index < self.items.len() {
            Some(self.items.remove(index))
        } else {
            None
        }
    }

    /// Lend a view of the items without giving them away.
    pub fn list(&self) -> &[String] {
        &self.items
    }
}

/// Tell the student which types copy and which move, using the compiler's own rule.
pub fn is_copy<T: Copy>() -> bool {
    // If this compiles, T is Copy. If it does not, T moves.
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primitives_are_copied_so_both_names_stay_valid() {
        let a = 5;
        let b = a; // copy, not move
        assert_eq!(a + b, 10);
        assert!(is_copy::<i32>());
        assert!(is_copy::<char>());
        assert!(is_copy::<bool>());
    }

    #[test]
    fn a_string_moves_unless_borrowed_or_cloned() {
        let owned = String::from("hi");
        let borrowed = &owned; // borrow: `owned` is still alive
        assert_eq!(length_of(borrowed), 2);
        assert_eq!(owned, "hi");

        let moved = owned.clone(); // explicit copy of the heap data
        assert_eq!(moved, owned);

        let given = take_then_return(owned.clone());
        assert_eq!(given, "hi");
    }

    #[test]
    fn a_bag_owns_what_is_put_in_it() {
        let mut bag = Bag::new("Ada");
        bag.add(String::from("compass"));
        bag.add(String::from("map"));
        assert_eq!(bag.list().len(), 2);

        let taken = bag.take(0).expect("index 0 exists");
        assert_eq!(taken, "compass");
        assert_eq!(bag.list(), ["map".to_string()], "the bag gave it away");
        assert_eq!(bag.take(9), None);
    }

    #[test]
    fn cloning_a_bag_gives_two_independent_bags() {
        let mut original = Bag::new("Ada");
        original.add(String::from("compass"));
        let mut copy = original.clone();
        copy.add(String::from("map"));

        assert_eq!(original.list().len(), 1);
        assert_eq!(copy.list().len(), 2);
    }
}
