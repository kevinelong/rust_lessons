# 01 — Comments and identifiers

<sub>rust_lessons 0.16.0 · source: `src/section_01_*.rs` · run: `cargo run --example 01_comments`</sub>

Section 1 — code is written for people.

The deck opens with the most important sentence in the course: *the primary
audience for source code is not the computer but ourselves and others who
will maintain it*. Names and comments are how we write for those readers.

Three ideas, and one hard rule for each:

* An **identifier** is a name we give a value. In Rust we write names in
  `snake_case` — `total_price`, not `TotalPrice` or `total price`.
* An identifier may contain letters, digits and underscores, but **may not
  start with a digit**, and it may not be a keyword such as `let` or `fn`.
* A **comment** is a note for the reader that the compiler ignores: `//` to
  the end of the line, `/* ... */` for a block. `///` is a doc comment, and
  Rust compiles those into the documentation this course is built from.

Keywords a variable may not be named.

Is `name` a legal Rust identifier under the rules above?

The rule is narrow on purpose: letters, digits and underscores, first
character not a digit, not empty, not a keyword. Comparing identifiers is
case sensitive, so `Total` and `total` are two different names.

A greeting built the way the course builds every message: from a name.

This function exists in every section from here on under different forms,
which is the point — the first program you write is still a program.

The comment styles Rust understands, as data, so the lesson can be tested.

