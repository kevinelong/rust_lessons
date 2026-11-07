# 18 — Errors: panic, Result and ?

<sub>rust_lessons 0.16.0 · source: `src/section_18_*.rs` · run: `cargo run --example 18_errors`</sub>

Section 18 — errors: `assert`, `panic!`, `Result` and `?`.

`python_lessons` teaches this as `assert_try_except.py`; the deck itself has no
slide for it. A program that cannot fail gracefully is a program that stops working
the first time the world misbehaves, so this belongs in the course regardless.

Rust has two levels, and the choice between them is a design decision, not a style
preference:

| Level | Written | Means | Use for |
|---|---|---|---|
| unrecoverable | `panic!`, `assert!`, `unwrap()` | this is a bug; stop | broken invariants, tests |
| recoverable | `Result<T, E>`, `?` | the caller must decide | missing files, bad input, network |

`?` is the sentence-maker: it either yields the value or returns the error to the
caller, so a function that may fail reads like the happy path.

# Diagram

`docs/diagrams/flow.md#-propagation` (Mermaid sequence) follows an error up three
frames and shows what the `?` skipped on the way.

A domain error, written out — the Rust answer to a `try`/`except ValueError`.

The quantity was not a positive number.

The item is not on the menu.

The total would overflow what we can charge.

The prices, in the smallest unit, as the only place prices are written.

Price `quantity` of `item`, refusing the three things that can be wrong.

Written with `?` throughout: every check is one line and the happy path is the last
expression, which is what `?` is for.

Add up a whole order, stopping at the first bad line.

`collect::<Result<Vec<_>, _>>()` is the idiom for "all or nothing": it returns the
first error rather than a partially built list.

The `expect` habit, in the one place it is appropriate: a value we control.

`unwrap()` panics with no context; `expect("why")` panics with the reason. In
library code neither belongs — the caller gets a `Result` instead.

