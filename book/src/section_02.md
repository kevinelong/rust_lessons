# 02 — Variables and data types

<sub>rust_lessons 0.17.0 · source: `src/section_02_*.rs` · run: `cargo run --example 02_variables`</sub>

Section 2 — variables and data types.

A variable is a labelled container for a value. Rust makes two things
explicit that Python and JavaScript leave implicit, and both are lessons the
deck already teaches:

* A variable is **immutable unless you say `mut`**. `let x = 5;` cannot be
  reassigned; `let mut x = 5;` can. You are told when your design assumed
  something would change.
* A variable **always has a type**, and the compiler infers it from the value
  when you do not write it: `let count = 3` is an `i32`.

`let` again with the same name is **shadowing**, not reassignment — the
second binding is a new variable that may even have a different type. That is
how you convert a value in place: `let text = "42"; let text = text.parse()?;`

The integer types, as (name, bits) — the deck's "integers are whole numbers".

The other basic types the deck introduces, by the name the deck uses.

The average of two integers — and the trap this lesson exists to show.

`(a + b) / 2` with two `i64` values performs **integer division**: the
fractional part is discarded, and `(1 + 2) / 2` is `1`, not `1.5`. Casting
both sides to `f64` first is the fix, and the reason the declaration says
`-> f64` while the inputs are integers.

The same average done the wrong way, kept only so tests can prove it differs.

A value converted by shadowing: text in, number out, same name.

