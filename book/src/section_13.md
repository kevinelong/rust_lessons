# 13 — Structs, methods and traits

<sub>rust_lessons 0.17.0 · source: `src/section_13_*.rs` · run: `cargo run --example 13_structs`</sub>

Section 13 — structs, methods and traits.

The deck's Alan Kay slide: objects as autonomous cells that talk to each other.
Rust has no classes, and it does the job with two pieces:

* a **struct** holds the properties (the deck's `self.name`),
* an **`impl` block** holds the methods,
* a **trait** is the shared contract — the closest thing to an interface, and
  the reason two unrelated types can be used the same way.

Rust keeps data and behaviour in separate declarations on purpose: you can see
exactly what a type stores before reading any of what it does. Encapsulation is
privacy (`pub` vs not), not a keyword on a class.

The contract every animal in the lesson satisfies.

What this animal says.

The species name, shown in the lesson's output.

A sentence composed from the two pieces above — a default method, so an
implementor only supplies the parts that differ.

The deck's `Dog` class: a name, a trick count, and methods.

A new dog that knows no tricks yet.

A property read through a method — the deck's "member variable".

A method that changes the object, hence `&mut self`.

A second type, to prove the trait is shared rather than special-cased.

Take anything that can speak — dynamic dispatch without a class hierarchy.

A bank account, the classic property/method example, with the invariant that a
balance never goes negative.

An account opens at zero.

The current balance in cents.

Add money; returns the new balance.

Take money out, refusing to overdraw.

