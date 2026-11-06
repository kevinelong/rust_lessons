# 17 — Ownership and moves

<sub>rust_lessons 0.12.0 · source: `src/section_17_*.rs` · run: `cargo run --example 17_ownership`</sub>

Section 17 — ownership and moves (Rust requires this; the deck never had to say it).

Sections 04, 07 and 13 borrow with `&` before this one explains *why* borrowing
exists. That order was a real gap: you cannot understand `&` until you know what it
avoids. This section pays that debt.

Every value has exactly **one owner**. When the owner goes out of scope the value is
freed — no garbage collector, no manual `free()`. Two consequences follow, and both
are compile errors rather than bugs:

* **Moving**: assigning or passing a value that owns its data (`String`, `Vec`)
  transfers ownership. The old name is dead; using it is an error.
* **Copying**: small fixed-size types (`i32`, `f64`, `bool`, `char`, a fixed-size
  array of them) are `Copy`, so passing them copies and the original stays usable.

`&` exists to avoid doing either: borrow instead of move, and keep the value.

# Diagrams

`docs/diagrams/ownership.svg` draws move, borrow and clone side by side;
`docs/diagrams/borrow-rules.svg` draws the count rule that makes them safe.

Take ownership of a `String` and give it back, so the lesson can show the hand-over.

Borrow a `String` and use it — the caller keeps their value, and this is why the
rest of the course writes `&str` in its signatures.

A struct that owns its data: moving it moves everything inside it.

A new bag.

Put something in, by value — the item is now the bag's.

Take something out, giving it away — after this the bag no longer has it.

Lend a view of the items without giving them away.

Tell the student which types copy and which move, using the compiler's own rule.

