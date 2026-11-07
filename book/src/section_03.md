# 03 — Operators and expressions

<sub>rust_lessons 0.16.0 · source: `src/section_03_*.rs` · run: `cargo run --example 03_operators`</sub>

Section 3 — operators and expressions.

The deck introduces `+ - * /` and parentheses, and calls the result of
combining variables and operators an **expression**. Rust adds two things
worth teaching straight away:

* `%` (remainder) and `&& / || / !` for the boolean operators.
* **Order of operations is not negotiable**: `2 + 3 * 4` is 14. When the
  intent is not obvious from reading it, parenthesise it — the compiler will
  not guess what you meant.

`a + b * c`, without parentheses, so the lesson can show what precedence does.

The same arithmetic with the order written down.

A different order, a different answer: `(a + b) * c`.

Celsius to Fahrenheit — the classic expression: `(c * 9 / 5) + 32`.

Order matters here too. Multiplying before dividing keeps the full precision
of the intermediate value; dividing first would lose it.

Even or odd, decided by the remainder operator.

The arithmetic operators the course uses, as (symbol, example, result).

