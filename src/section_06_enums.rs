//! Section 6 — enums: making your own type from a fixed set of values.
//!
//! The deck's `Shake(Enum)` slide: an enumeration defines a type whose value is
//! one of a named set. Rust's `enum` is the strongest version of this idea in any
//! of the course languages, because the variants can carry data and because
//! `match` must be exhaustive — forget a case and the code does not compile.
//!
//! This is where the design instinct to use *a set of words instead of a magic
//! number* starts: `Shake::Vanilla`, not `0`.

//! # Diagram
//!
//! `docs/diagrams/match-exhaustiveness.svg` shows why an arm per variant, and not a
//! wildcard, is what keeps a new variant from pricing itself at zero.
//!

/// The deck's ice-cream example, with a price attached to each flavour.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shake {
    Vanilla,
    Chocolate,
    Strawberry,
}

impl Shake {
    /// Price in cents. `match` on `self` must list every variant.
    pub fn price_cents(self) -> u32 {
        match self {
            Shake::Vanilla => 350,
            Shake::Chocolate => 400,
            Shake::Strawberry => 375,
        }
    }

    /// The word to print, so the lesson can convert a variant to text.
    pub fn label(self) -> &'static str {
        match self {
            Shake::Vanilla => "vanilla",
            Shake::Chocolate => "chocolate",
            Shake::Strawberry => "strawberry",
        }
    }

    /// Every variant, which is what makes the type testable as a set.
    pub fn all() -> [Shake; 3] {
        [Shake::Vanilla, Shake::Chocolate, Shake::Strawberry]
    }
}

/// An enum whose variants carry different data — the feature Python's `Enum` does
/// not have and the reason Rust code prefers enums to class hierarchies.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Payment {
    Cash(u32),
    Card { last4: u32, cents: u32 },
    Refunded(&'static str),
}

impl Payment {
    /// How much money changed hands; a refund is negative.
    pub fn net_cents(self) -> i64 {
        match self {
            Payment::Cash(cents) => cents as i64,
            Payment::Card { cents, .. } => cents as i64,
            Payment::Refunded(_) => -1,
        }
    }
}

/// `Option<T>` is the built-in enum you have already been using: either `Some`
/// value or `None`. This function shows the pattern instead of the wiring.
pub fn half_of(n: i64) -> Option<i64> {
    if n % 2 == 0 {
        Some(n / 2)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_variant_has_a_price_and_a_label() {
        let prices: Vec<u32> = Shake::all().iter().map(|s| s.price_cents()).collect();
        assert_eq!(prices, [350, 400, 375]);
        assert_eq!(Shake::Chocolate.label(), "chocolate");
    }

    #[test]
    fn variants_can_carry_data() {
        assert_eq!(Payment::Cash(500).net_cents(), 500);
        assert_eq!(Payment::Card { last4: 4242, cents: 250 }.net_cents(), 250);
        assert_eq!(Payment::Refunded("duplicate charge").net_cents(), -1);
    }

    #[test]
    fn option_is_an_enum_too() {
        assert_eq!(half_of(10), Some(5));
        assert_eq!(half_of(7), None);
    }
}
