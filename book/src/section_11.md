# 11 — Functions

<sub>rust_lessons 0.18.0 · source: `src/section_11_*.rs` · run: `cargo run --example 11_anatomy_of_a_function`</sub>

Section 11 — functions.

# Subgoals

1. **Write the signature from the question, not from the body.** What goes in, what comes
   out, and can it fail (`Option`/`Result`) — the deck's exercises are decided here.
2. **Choose the ownership of each parameter.** `&str`/`&[i32]` when borrowing is enough;
   by value only when the function keeps it.
3. **Write the body as one traversal.** `min`/`max`/`roll` each walk their input once.
4. **State the acceptance case.** Every exercise in this section has a test that names
   the boundary (empty input, all faces, non-ASCII text) — that test is the definition of
   done, not an optional extra.
The deck's D.R.Y. slide, then the six exercises that make it real: `min`, `max`,
`roll`, `get_average_roll`, `trackAverage`, `chart_average` and `reverse_me`.
All of them live here, in Rust, and the tests at the bottom are the deck's
acceptance criteria.

# Anatomy of a function

Read a signature left to right, and every token has exactly one job:

```text
pub  fn  describe_dog  (name: &str, age: u8)  ->  String  {  format!(...)  }
│    │   │              │                     │   │       │  │
│    │   │              │                     │   │       │  └ body: an expression, whose value is returned
│    │   │              │                     │   │       └ brace opens the body
│    │   │              │                     │   └ the type of the returned value
│    │   │              │                     └ arrow, read as "returns"
│    │   │              └ parameter list: a pattern, a colon, a type, comma
│    │   └ the name, in snake_case (section 01's rule)
│    └ the keyword that starts a function
└ visibility: `pub` makes it callable outside this module
```

Three forms share that grammar, and the difference is only what is in the
parentheses:

| Form | Written | Called as | Has `self` |
|---|---|---|---|
| function | `fn describe(name: &str) -> String` | `describe("Rex")` | no |
| associated function | `fn new(name: &str) -> Self` | `Dog::new("Rex")` | no |
| method | `fn rename(&mut self, to: &str)` | `dog.rename("Rex")` | yes, borrowed |

A function that returns nothing says so by returning `()`, which is written by
omitting the arrow. A function that never returns at all has the return type
`!`, the never type — useful for `panic!`-style helpers and for a loop that can
only be left by a signal.

The symbols in those signatures are introduced where they first appear and
collected in one table in `docs/symbols.md`: `&` and `&mut` (borrow), `->`
(returns), `::` (path), `|x|` (closure), `_` (deliberately unused), `!` (never
type, and the marker that makes `println!` a macro rather than a function).

A Rust function declares its parameters and its return type, and the body's
last expression is the return value — no `return` needed unless you are leaving
early. Arguments of `&str` and `&[i32]` are *borrowed*, which is the closest
thing Rust has to "pass by reference" and the reason none of these functions
copy their input.

A tiny deterministic random number generator.

`rand` would be a dependency; a linear congruential generator is forty lines of
arithmetic and makes every dice exercise here reproducible, which matters when
the output is a test.

A generator seeded to a fixed value — the same seed gives the same rolls.

The next value, widened from a 64-bit state by the usual LCG constants.

A number in `1..=sides`, which is what a die shows.

`min` — smallest of a slice, or `None` when empty.

`max` — largest of a slice, or `None` when empty.

`roll` — total for `quantity` dice with `sides` faces each.

`get_average_roll` — roll `quantity` dice `times` times and average the totals.

`trackAverage` — count how often each *die face* comes up across many rolls.

`chart_average` — the same data as a horizontal bar chart, scaled to `width`.

`reverse_me` — the deck's "without the `[::-1]` shortcut" exercise.

`chars().rev()` is the Rust idiomatic answer and, unlike reversing bytes,
it is correct for text that is not ASCII.

