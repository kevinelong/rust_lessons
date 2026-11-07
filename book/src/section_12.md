# 12 — Capstone: guess the number

<sub>rust_lessons 0.16.0 · source: `src/section_12_*.rs` · run: `cargo run --example 12_capstone`</sub>

Section 12 — capstone: guess the number.

The deck says that after the first eleven sections "we have now learned enough
to write something useful and interesting", and then builds a guess-the-number
game. This is that game, split so the logic is testable and the input handling
stays in `examples/12_capstone.rs`.

The shape is a small state machine: a secret, a count of attempts, and a
`guess` method that returns whether the guess was low, high or exact. The tests
at the bottom play whole games without a keyboard.

The range only ever shrinks: `remaining_range` returns the smallest and largest value
still possible, and each miss moves one end inward, so a game that is played sensibly
cannot go on forever.

One game of guess-the-number.

What a guess turned out to be.

A new game with the secret inside the range `1..=high`.

The secret, for the tests and for showing the answer at the end.

How many guesses have been made.

The smallest value that is still possible — the range narrows with play.

Play one guess.

`Err(attempts)` means the game was won on that attempt; `Ok(Ordering)` says
the guess was low or high and the player may try again. Returning the count
only on the winning branch is what makes `?` usable by a caller.

A hint written the way the game narrates it.

Play a scripted game with a list of guesses, the way a test or a replay would.

Returns the number of attempts used, or the count of guesses made if the script
never wins.

The best first guess for a range of `1..=high`, by halving it.

