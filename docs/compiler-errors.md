# Reading compiler errors

The transcripts below are **captured from `rustc 1.96.0` on this machine**, not remembered:
each program was compiled, its stderr recorded verbatim, and nothing was edited. Reproduce
any of them with `rustc --edition 2021 <file>.rs`.

## How to read one

```
error[E0308]: mismatched types        <- 1. code and one-line summary
 --> e0308.rs:2:23                   <- 2. file:line:column of the problem
  |
2 |     let count: i32 = "three";   <- 3. the source line, quoted back
  |                ^^^^^^^ expected `i32`, found `&str`   <- 4. the span and what it is
  |
  = note: ...                        <- 5. why, or what would make it work
```

1. **The code** is stable and searchable — `E0308` means the same thing in every project.
2. **The location** is where the compiler *concluded* the mismatch, which is not always
   where the mistake is. Read the next line before editing.
3. **The quoted source line** saves you finding it.
4. **The span** (`^^^^`) marks the exact expression the compiler is talking about.
5. **The note** is the part beginners skip and the part that most often contains the fix.

The habit worth building: read the code, then the span, then the note — and only then touch
the file. The five below are the ones a Rust beginner meets in the first week.

## E0308 — Mismatched types

The value on the right is text; the annotation on the left says number. The error points at the *annotation*, because that is what the compiler believes you meant.

```rust
fn main() {
    let count: i32 = "three";
    println!("{}", count);
}
```

Captured output:

```text
error[E0308]: mismatched types
 --> e0308_mismatched_types.rs:2:22
  |
2 |     let count: i32 = "three";
  |                ---   ^^^^^^^ expected `i32`, found `&str`
  |                |
  |                expected due to this

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
```

## E0382 — Use of moved value

`name` moved into `owned`; the second use is a read of something that no longer exists. The note about `Copy` is the compiler telling you why this type was not copied.

```rust
fn main() {
    let name = String::from("Ada");
    let owned = name;
    println!("{} {}", owned, name);
}
```

Captured output:

```text
error[E0382]: borrow of moved value: `name`
 --> e0382_use_after_move.rs:4:30
  |
2 |     let name = String::from("Ada");
  |         ---- move occurs because `name` has type `String`, which does not implement the `Copy` trait
3 |     let owned = name;
  |                 ---- value moved here
4 |     println!("{} {}", owned, name);
  |                              ^^^^ value borrowed here after move
  |
help: consider cloning the value if the performance cost is acceptable
  |
3 |     let owned = name.clone();
  |                     ++++++++

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0382`.
```

## E0499 — Two mutable borrows

Two `&mut` to the same value. Either take them one at a time, or use one borrow twice.

```rust
fn main() {
    let mut text = String::from("hi");
    let a = &mut text;
    let b = &mut text;
    println!("{} {}", a, b);
}
```

Captured output:

```text
error[E0499]: cannot borrow `text` as mutable more than once at a time
 --> e0499_two_mutable_borrows.rs:4:13
  |
3 |     let a = &mut text;
  |             --------- first mutable borrow occurs here
4 |     let b = &mut text;
  |             ^^^^^^^^^ second mutable borrow occurs here
5 |     println!("{} {}", a, b);
  |                       - first borrow later used here

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0499`.
```

## E0004 — Non-exhaustive patterns

A `match` that does not cover every variant. The help names the missing arm, which is the whole point of the check.

```rust
enum Shake { Vanilla, Chocolate, Strawberry }
fn main() {
    let s = Shake::Vanilla;
    let price = match s {
        Shake::Vanilla => 350,
        Shake::Chocolate => 400,
    };
    println!("{}", price);
}
```

Captured output:

```text
error[E0004]: non-exhaustive patterns: `Shake::Strawberry` not covered
 --> e0004_non_exhaustive_match.rs:4:23
  |
4 |     let price = match s {
  |                       ^ pattern `Shake::Strawberry` not covered
  |
note: `Shake` defined here
 --> e0004_non_exhaustive_match.rs:1:6
  |
1 | enum Shake { Vanilla, Chocolate, Strawberry }
  |      ^^^^^                       ---------- not covered
  = note: the matched value is of type `Shake`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
  |
6 ~         Shake::Chocolate => 400,
7 ~         Shake::Strawberry => todo!(),
  |

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0004`.
```

## E0061 — Wrong number of arguments

The signature wants two arguments, the call passes one. The note lists the expected arguments in order.

```rust
fn greet(name: &str, greeting: &str) -> String {
    format!("{greeting}, {name}")
}
fn main() {
    println!("{}", greet("Ada"));
}
```

Captured output:

```text
error[E0061]: this function takes 2 arguments but 1 argument was supplied
 --> e0061_wrong_argument_count.rs:5:20
  |
5 |     println!("{}", greet("Ada"));
  |                    ^^^^^------- argument #2 of type `&str` is missing
  |
note: function defined here
 --> e0061_wrong_argument_count.rs:1:4
  |
1 | fn greet(name: &str, greeting: &str) -> String {
  |    ^^^^^             --------------
help: provide the argument
  |
5 |     println!("{}", greet("Ada", /* &str */));
  |                               ++++++++++++

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0061`.
```

## The fixes

Each of these is a compiling function in `src/section_21_compiler_errors.rs` with a test, so
the corrected form is not just described — it runs.
