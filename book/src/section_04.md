# 04 — Strings, numbers and conversion

<sub>rust_lessons 0.17.0 · source: `src/section_04_*.rs` · run: `cargo run --example 04_strings`</sub>

Section 4 — strings, numbers, and converting between them.

The deck's most-used data type is text, and its most common operation is
*conversion*: a number turned into text to print, text turned into a number to
calculate with.

Rust splits what other languages call "a string" in two, and it is worth the
trouble to learn early:

* `&str` is **borrowed** text — a view into characters someone else owns. For a string
  literal such as `"hi"`, the owner is the compiled binary: the view lives for the whole
  program (`&'static str`), which is why literals need no lifetime of their own. What
  "borrowed" *means* — and how a borrow differs from a move — is section 17; here it is
  only a name for "a view, not the text itself".
* `String` is **owned** text you can grow. `String::from("x")`, `.push_str()`,
  `format!`.

# Diagram

`docs/diagrams/string-stack-heap.svg` draws the two words a `String` keeps on the
stack and the bytes it points at on the heap.

Conversion is explicit and can fail, so `parse` returns a `Result`: text that
is not a number is an error you must handle, not a silent `NaN`.
Uppercase text with an exclamation mark — the deck's `twice()` example's cousin.

Repeat text, the `twice()` exercise from the deck: `&str` in, owned `String` out.

Add two numbers written as text. `None` when either is not a number.

This is the honest version of "convert a string to a number": the failure is
part of the type, so a caller cannot forget it.

A number rendered back into text with a thousands separator, to show that
going the other way is lossless.

The number of `char`s — not bytes. `"hi"` is 2, `"héllo"` is 5 while its
byte length is 6, which is the difference this lesson points out.

