# 09 — Ranges and iterators

<sub>rust_lessons 0.12.0 · source: `src/section_09_*.rs` · run: `cargo run --example 09_iterators`</sub>

Section 9 — ranges and iterators.

The deck's "ranges may contain an infinite number of items; they save memory by
generating each item as we iterate over them" is exactly what a Rust iterator
is. `1..=1000` does not build a list of a thousand numbers; it is a recipe for
producing them one at a time.

An iterator is lazy: `map` and `filter` describe work and do none of it until
something consumes the chain — `collect`, `sum`, `count`, `for`. That laziness
is why `(1..).map(...).take(5)` terminates and `(1..).collect::<Vec<_>>()` does
not.

# Diagram

`docs/diagrams/flow.md#iterator-laziness` (Mermaid): lazy stages on the left, the
consuming stage that actually runs them on the right.

The even numbers from 2 up to `n`, inclusive.

Total a slice with an iterator rather than a loop.

How many values exceed `limit` — `filter` then `count`.

The first `n` squares, from an infinite range: laziness, made visible.

Running totals — `scan` folds and keeps each intermediate value.

Words longer than three characters, uppercased — a chain that reads like the
English description of the job.

