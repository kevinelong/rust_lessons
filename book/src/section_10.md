# 10 — Loops

<sub>rust_lessons 0.16.0 · source: `src/section_10_*.rs` · run: `cargo run --example 10_loops`</sub>

Section 10 — loops.

The deck splits loops into two jobs: a **`for`** loop that visits each item of a
collection, and a **`while`** loop that repeats until a condition is met. Rust
has both plus a third, `loop`, for the case where the exit is somewhere in the
middle of the body.

Two controls work in all three: `break` leaves the loop early, `continue`
skips to the next turn. In Rust `break` can also carry a value out of a `loop`,
which is how you write "search until you find it" without a flag variable.

FizzBuzz, the exercise every course uses: multiples of 3 say `Fizz`, of 5 say
`Buzz`, of both say `FizzBuzz`, everything else is the number as text.

How many steps for the Collatz sequence to reach 1 — a `while` loop that must
terminate but proves nothing about stopping at 1 for every input.

The index of the first value over the limit, found with `loop` + `break value`.

Count the vowels with a `for` loop and `continue`, to show the skip.

