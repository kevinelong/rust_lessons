# 05 — Booleans and conditionals

<sub>rust_lessons 0.19.0 · source: `src/section_05_*.rs` · run: `cargo run --example 05_conditionals`</sub>

Section 5 — booleans and conditionals.

A `bool` is `true` or `false`, and is usually *produced* by comparing two
values rather than typed by hand: `==`, `!=`, `<`, `<=`, `>`, `>=`.

The deck's rule of thumb is worth restating in Rust because the compiler
enforces it: **the condition of an `if` must be a `bool`**. There is no
truthiness — `if count { }` is an error, and `if count > 0 { }` is the fix.

Rust's `if` is an expression: it produces a value, so both branches must have
the same type.

Grade a score the way the deck's exercise does, with an `if`/`else` chain.

The sign of a number, using `if` as an expression.

Boolean operators: `&&` and, `||` or, `!` not. Note that `&&` short-circuits,
so the right-hand side is not evaluated when the left already decides it.

The comparison operators, as (symbol, meaning).

