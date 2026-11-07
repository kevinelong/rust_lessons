# 21 — Reading compiler errors

<sub>rust_lessons 0.17.0 · source: `src/section_21_*.rs` · run: `cargo run --example 21_compiler_errors`</sub>

Section 21 — reading compiler errors.

The CMU study of Rust learners puts compiler messages near the top of the barrier list,
and the Rust cohorts in the earlier courses never covered it: the Ruby track had a lesson
called *reading error messages* and the Python deck had none. So this section exists,
and its transcripts are **captured from `rustc 1.96.0`**, not remembered — see
`docs/compiler-errors.md` for the five full outputs, verbatim.

What an error contains, in the order you should read it:

1. **the code** (`E0308`) — stable and searchable;
2. **the location** — where the compiler *concluded* the problem, which is not always
   where the mistake is;
3. **the quoted source line**;
4. **the span** (`^^^^`) — the exact expression under discussion;
5. **the note** — the part beginners skip and the part that usually contains the fix.

Each function below is the *corrected* form of one captured error, with the test that
proves it works. The broken form is not in this module: a crate that does not compile
cannot teach, which is why the broken inputs live in the notes and the fixes live here.

**E0308, mismatched types.** `let count: i32 = "three";`

The annotation promised a number and the value was text. The fix is to convert, not to
delete the annotation: `parse` returns a `Result` because `"three"` cannot become a number.

**E0382, use of moved value.** `let owned = name;` then using `name`.

`String` is not `Copy`: assigning moved the buffer and the old name is dead. Borrow when
you only need to read, clone when you truly need two owners, and say which in a comment —
a clone that is not explained is usually a misunderstanding.

**E0499, two mutable borrows.** `let a = &mut text; let b = &mut text;`

At most one `&mut` may exist at a time. The fix is to use one borrow twice in sequence,
which is exactly what the borrow checker is asking for.

**E0004, non-exhaustive patterns.** a `match` missing `Strawberry`.

The compiler names the missing arm. Completing the enum is the fix; a `_ => 0` arm would
compile and would silently price a future variant at zero — which is why the course never
uses a catch-all for a domain enum.

**E0061, wrong number of arguments.** `greet("Ada")` against `greet(name, greeting)`.

The note lists the expected arguments in order. The fix is to supply both — and, if the
second is usually the same, to give it a default by taking `Option` or by offering a
two-argument wrapper rather than changing every call site.

The single-argument form `E0061` was asking for, kept as a named convenience.

