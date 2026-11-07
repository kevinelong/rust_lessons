# 06 — Enums: making your own type

<sub>rust_lessons 0.20.0 · source: `src/section_06_*.rs` · run: `cargo run --example 06_enums`</sub>

Section 6 — enums: making your own type from a fixed set of values.

The deck's `Shake(Enum)` slide: an enumeration defines a type whose value is
one of a named set. Rust's `enum` is the strongest version of this idea in any
of the course languages, because the variants can carry data and because
`match` must be exhaustive — forget a case and the code does not compile.

This is where the design instinct to use *a set of words instead of a magic
number* starts: `Shake::Vanilla`, not `0`.

# Diagram

`docs/diagrams/match-exhaustiveness.svg` shows why an arm per variant, and not a
wildcard, is what keeps a new variant from pricing itself at zero.

The deck's ice-cream example, with a price attached to each flavour.

Price in cents. `match` on `self` must list every variant.

The word to print, so the lesson can convert a variant to text.

Every variant, which is what makes the type testable as a set.

An enum whose variants carry different data — the feature Python's `Enum` does
not have and the reason Rust code prefers enums to class hierarchies.

How much money changed hands; a refund is negative.

`Option<T>` is the built-in enum you have already been using: either `Some`
value or `None`. This function shows the pattern instead of the wiring.

