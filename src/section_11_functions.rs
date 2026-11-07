//! Section 11 — functions.
//!
//! # Subgoals
//!
//! 1. **Write the signature from the question, not from the body.** What goes in, what comes
//!    out, and can it fail (`Option`/`Result`) — the deck's exercises are decided here.
//! 2. **Choose the ownership of each parameter.** `&str`/`&[i32]` when borrowing is enough;
//!    by value only when the function keeps it.
//! 3. **Write the body as one traversal.** `min`/`max`/`roll` each walk their input once.
//! 4. **State the acceptance case.** Every exercise in this section has a test that names
//!    the boundary (empty input, all faces, non-ASCII text) — that test is the definition of
//!    done, not an optional extra.
//!
//! # Worked example — the longest word in a sentence
//!
//! "Which word is the longest?" is four decisions before it is a line of code, and
//! each one removes a way of getting it wrong.
//!
//! 1. **Write the signature from the question, not from the body.** In: a sentence.
//!    Out: one word. Can it fail: yes, `""` holds no word — so `Option<&str>`.
//!    That signature is the answer; the body only fills it in.
//! 2. **Choose the ownership of each parameter.** `&str` borrows the caller's
//!    text, and the returned word borrows from it too — it is a slice of the
//!    sentence, so nothing is copied and no lifetime has to be written down.
//! 3. **Write the body as one traversal.** `split_whitespace` walks the text once,
//!    and `max_by_key` keeps the longest of what it saw. No second pass, no
//!    intermediate `Vec` of words.
//! 4. **State the acceptance case.** The boundary cases are the test: `""` gives
//!    `None`, and a tie goes to the last word reached — `"ab cd"` gives `"cd"` —
//!    which is written down here instead of discovered later.
//!
//! ```text
//! fn longest_word(text: &str) -> Option<&str> {
//!     text.split_whitespace().max_by_key(|w| w.len())
//! }
//! println!("{:?}", longest_word("the quick red fox")); // Output: Some("quick")
//! println!("{:?}", longest_word("ab cd"));             // Output: Some("cd")
//! println!("{:?}", longest_word(""));                  // Output: None
//! ```
//!
//! The deck's D.R.Y. slide, then the six exercises that make it real: `min`, `max`,
//! `roll`, `get_average_roll`, `trackAverage`, `chart_average` and `reverse_me`.
//! All of them live here, in Rust, and the tests at the bottom are the deck's
//! acceptance criteria.
//!
//! # Anatomy of a function
//!
//! Read a signature left to right, and every token has exactly one job:
//!
//! ```text
//! pub  fn  describe_dog  (name: &str, age: u8)  ->  String  {  format!(...)  }
//! │    │   │              │                     │   │       │  │
//! │    │   │              │                     │   │       │  └ body: an expression, whose value is returned
//! │    │   │              │                     │   │       └ brace opens the body
//! │    │   │              │                     │   └ the type of the returned value
//! │    │   │              │                     └ arrow, read as "returns"
//! │    │   │              └ parameter list: a pattern, a colon, a type, comma
//! │    │   └ the name, in snake_case (section 01's rule)
//! │    └ the keyword that starts a function
//! └ visibility: `pub` makes it callable outside this module
//! ```
//!
//! Three forms share that grammar, and the difference is only what is in the
//! parentheses:
//!
//! | Form | Written | Called as | Has `self` |
//! |---|---|---|---|
//! | function | `fn describe(name: &str) -> String` | `describe("Rex")` | no |
//! | associated function | `fn new(name: &str) -> Self` | `Dog::new("Rex")` | no |
//! | method | `fn rename(&mut self, to: &str)` | `dog.rename("Rex")` | yes, borrowed |
//!
//! A function that returns nothing says so by returning `()`, which is written by
//! omitting the arrow. A function that never returns at all has the return type
//! `!`, the never type — useful for `panic!`-style helpers and for a loop that can
//! only be left by a signal.
//!
//! The symbols in those signatures are introduced where they first appear and
//! collected in one table in `docs/symbols.md`: `&` and `&mut` (borrow), `->`
//! (returns), `::` (path), `|x|` (closure), `_` (deliberately unused), `!` (never
//! type, and the marker that makes `println!` a macro rather than a function).
//!
//! A Rust function declares its parameters and its return type, and the body's
//! last expression is the return value — no `return` needed unless you are leaving
//! early. Arguments of `&str` and `&[i32]` are *borrowed*, which is the closest
//! thing Rust has to "pass by reference" and the reason none of these functions
//! copy their input.

/// A tiny deterministic random number generator.
///
/// `rand` would be a dependency; a linear congruential generator is forty lines of
/// arithmetic and makes every dice exercise here reproducible, which matters when
/// the output is a test.
#[derive(Debug, Clone)]
pub struct Rng {
    state: u64,
}

impl Rng {
    /// A generator seeded to a fixed value — the same seed gives the same rolls.
    pub fn new(seed: u64) -> Self {
        Rng {
            state: seed | 1, // never zero, which would freeze the sequence
        }
    }

    /// The next value, widened from a 64-bit state by the usual LCG constants.
    fn next_u64(&mut self) -> u64 {
        self.state = self
            .state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.state
    }

    /// A number in `1..=sides`, which is what a die shows.
    pub fn roll_die(&mut self, sides: u32) -> u32 {
        debug_assert!(sides > 0, "a die needs at least one side");
        (self.next_u64() % sides as u64) as u32 + 1
    }
}

/// `min` — smallest of a slice, or `None` when empty.
pub fn min(values: &[i32]) -> Option<i32> {
    let mut best: Option<i32> = None;
    for &v in values {
        best = Some(match best {
            None => v,
            Some(b) if v < b => v,
            Some(b) => b,
        });
    }
    best
}

/// `max` — largest of a slice, or `None` when empty.
pub fn max(values: &[i32]) -> Option<i32> {
    let mut best: Option<i32> = None;
    for &v in values {
        best = Some(match best {
            None => v,
            Some(b) if v > b => v,
            Some(b) => b,
        });
    }
    best
}

/// `roll` — total for `quantity` dice with `sides` faces each.
pub fn roll(quantity: u32, sides: u32, rng: &mut Rng) -> u32 {
    (0..quantity).map(|_| rng.roll_die(sides)).sum()
}

/// `get_average_roll` — roll `quantity` dice `times` times and average the totals.
pub fn average_roll(quantity: u32, sides: u32, times: u32, rng: &mut Rng) -> f64 {
    if times == 0 {
        return 0.0;
    }
    let total: u32 = (0..times).map(|_| roll(quantity, sides, rng)).sum();
    total as f64 / times as f64
}

/// `trackAverage` — count how often each *die face* comes up across many rolls.
pub fn track_average(quantity: u32, sides: u32, times: u32, rng: &mut Rng) -> Vec<(u32, u32)> {
    let mut buckets = vec![0u32; sides as usize];
    for _ in 0..times {
        for _ in 0..quantity {
            let face = rng.roll_die(sides);
            buckets[face as usize - 1] += 1;
        }
    }
    buckets
        .into_iter()
        .enumerate()
        .map(|(i, n)| (i as u32 + 1, n))
        .collect()
}

/// `chart_average` — the same data as a horizontal bar chart, scaled to `width`.
pub fn chart_average(data: &[(u32, u32)], width: usize) -> String {
    let peak = data.iter().map(|(_, n)| *n).max().unwrap_or(0);
    let mut out = String::new();
    for (face, count) in data {
        let bars = if peak == 0 {
            0
        } else {
            (*count as usize * width) / peak as usize
        };
        out.push_str(&format!("{face:>3} |{:<width$}| {count}\n", "#".repeat(bars), width = width));
    }
    out
}

/// `reverse_me` — the deck's "without the `[::-1]` shortcut" exercise.
///
/// `chars().rev()` is the Rust idiomatic answer and, unlike reversing bytes,
/// it is correct for text that is not ASCII.
pub fn reverse_me(text: &str) -> String {
    text.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_and_max_agree_with_the_stdlib() {
        let v = [3, 2, 1, 2];
        assert_eq!(min(&v), Some(1));
        assert_eq!(max(&v), Some(3));
        assert_eq!(min(&[]), None);
        assert_eq!(max(&[]), None);
    }

    #[test]
    fn a_roll_stays_within_the_faces() {
        let mut rng = Rng::new(7);
        for _ in 0..200 {
            let r = roll(3, 6, &mut rng);
            assert!((3..=18).contains(&r), "3d6 must be 3..=18, got {r}");
        }
    }

    #[test]
    fn the_same_seed_replays_the_same_rolls() {
        let mut a = Rng::new(42);
        let mut b = Rng::new(42);
        let rolls_a: Vec<u32> = (0..10).map(|_| a.roll_die(6)).collect();
        let rolls_b: Vec<u32> = (0..10).map(|_| b.roll_die(6)).collect();
        assert_eq!(rolls_a, rolls_b);
        assert!(rolls_a.iter().all(|r| (1..=6).contains(r)));
    }

    #[test]
    fn averages_and_tallies_cover_every_face() {
        let mut rng = Rng::new(2024);
        let avg = average_roll(2, 6, 100, &mut rng);
        assert!((2.0..=12.0).contains(&avg), "2d6 average out of range: {avg}");

        let tally = track_average(1, 6, 600, &mut rng);
        assert_eq!(tally.len(), 6, "one bucket per face");
        assert_eq!(tally.iter().map(|(_, n)| n).sum::<u32>(), 600);
        assert!(tally.iter().all(|(_, n)| *n > 0), "600 rolls should touch every face");

        let chart = chart_average(&tally, 20);
        assert_eq!(chart.lines().count(), 6);
        assert!(chart.contains('|'));
    }

    #[test]
    fn reverse_works_on_text_that_is_not_ascii() {
        assert_eq!(reverse_me("abc"), "cba");
        assert_eq!(reverse_me("héllo"), "olléh");
        assert_eq!(reverse_me(""), "");
    }
}
