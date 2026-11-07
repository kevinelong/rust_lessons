//! Section 12 — capstone: guess the number.
//!
//! The deck says that after the first eleven sections "we have now learned enough
//! to write something useful and interesting", and then builds a guess-the-number
//! game. This is that game, split so the logic is testable and the input handling
//! stays in `examples/12_capstone.rs`.
//!
//! The shape is a small state machine: a secret, a count of attempts, and a
//! `guess` method that returns whether the guess was low, high or exact. The tests
//! at the bottom play whole games without a keyboard.
//!
//! The range only ever shrinks: `remaining_range` returns the smallest and largest value
//! still possible, and each miss moves one end inward, so a game that is played sensibly
//! cannot go on forever.

use std::cmp::Ordering;

/// One game of guess-the-number.
#[derive(Debug, Clone)]
pub struct Game {
    secret: u32,
    guesses: u32,
    low: u32,
    high: u32,
}

/// What a guess turned out to be.
pub type Verdict = Result<Ordering, u32>;

impl Game {
    /// A new game with the secret inside the range `1..=high`.
    pub fn new(secret: u32, high: u32) -> Self {
        assert!(secret >= 1 && secret <= high, "secret must be inside 1..={high}");
        Game {
            secret,
            guesses: 0,
            low: 1,
            high,
        }
    }

    /// The secret, for the tests and for showing the answer at the end.
    pub fn secret(&self) -> u32 {
        self.secret
    }

    /// How many guesses have been made.
    pub fn guesses(&self) -> u32 {
        self.guesses
    }

    /// The smallest value that is still possible — the range narrows with play.
    pub fn remaining_range(&self) -> (u32, u32) {
        (self.low, self.high)
    }

    /// Play one guess.
    ///
    /// `Err(attempts)` means the game was won on that attempt; `Ok(Ordering)` says
    /// the guess was low or high and the player may try again. Returning the count
    /// only on the winning branch is what makes `?` usable by a caller.
    pub fn guess(&mut self, value: u32) -> Verdict {
        self.guesses += 1;
        match value.cmp(&self.secret) {
            Ordering::Equal => Err(self.guesses),
            Ordering::Less => {
                self.low = self.low.max(value + 1);
                Ok(Ordering::Less)
            }
            Ordering::Greater => {
                self.high = self.high.min(value.saturating_sub(1));
                Ok(Ordering::Greater)
            }
        }
    }

    /// A hint written the way the game narrates it.
    pub fn hint(ordering: Ordering) -> &'static str {
        match ordering {
            Ordering::Less => "too low",
            Ordering::Greater => "too high",
            Ordering::Equal => "you got it",
        }
    }
}

/// Play a scripted game with a list of guesses, the way a test or a replay would.
///
/// Returns the number of attempts used, or the count of guesses made if the script
/// never wins.
pub fn play(secret: u32, guesses: &[u32]) -> Result<u32, u32> {
    let mut game = Game::new(secret, 100);
    for g in guesses {
        if let Err(attempts) = game.guess(*g) {
            return Ok(attempts);
        }
    }
    Err(game.guesses())
}

/// The best first guess for a range of `1..=high`, by halving it.
pub fn opening_guess(high: u32) -> u32 {
    (high / 2).max(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_game_can_be_won() {
        assert_eq!(play(42, &[50, 25, 42]), Ok(3));
    }

    #[test]
    fn a_script_that_never_wins_reports_its_length() {
        assert_eq!(play(42, &[1, 2, 3]), Err(3));
    }

    #[test]
    fn hints_narrow_the_range() {
        let mut g = Game::new(42, 100);
        assert_eq!(g.guess(50), Ok(Ordering::Greater));
        assert_eq!(g.remaining_range(), (1, 49));
        assert_eq!(g.guess(25), Ok(Ordering::Less));
        assert_eq!(g.remaining_range(), (26, 49));
        assert_eq!(g.guess(42), Err(3));
        assert_eq!(g.remaining_range().0 <= g.secret() && g.remaining_range().1 >= g.secret(), true);
    }

    #[test]
    fn binary_search_wins_within_seven_tries_up_to_one_hundred() {
        for secret in [1, 42, 73, 100] {
            let mut g = Game::new(secret, 100);
            let mut guess = opening_guess(100);
            let mut attempts = 0;
            loop {
                attempts += 1;
                match g.guess(guess) {
                    Err(n) => {
                        assert_eq!(n, attempts, "the win reports the attempt count");
                        assert!(attempts <= 7, "a hundred values need at most seven halvings");
                        break;
                    }
                    Ok(_) => {
                        let (low, high) = g.remaining_range();
                        guess = (low + high) / 2;
                    }
                }
            }
        }
    }
}
