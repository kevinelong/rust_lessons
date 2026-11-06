# Symbols used in this course

Rust is punctuation-heavy, and every symbol below is introduced where the course first
needs it. This is the index: the symbol, what it does, the first place it appears, and
one line of the course's own code that uses it.

The rule the course follows: **a symbol is taught the first time it is required**, not
in a preliminary chapter — and no example uses a symbol the student has not met.

## The ones that come first

| Symbol | Read as | First taught | Example from the course |
|---|---|---|---|
| `//` | comment to end of line | 01 | `// Comments explain *why*, not *what*` |
| `/* */` | block comment | 01 | `/* block comment */` |
| `///` | doc comment, compiled into the docs | 01 | `/// Is \`name\` a legal Rust identifier?` |
| `=` | bind a value to a name | 02 | `let quantity = 144;` |
| `mut` | "this binding may change" | 02 | `let mut mutable = 5;` |
| `:` | "has type" | 02 | `let value: i64 = value.parse().unwrap();` |
| `+ - * / %` | arithmetic, remainder | 03 | `(a as f64 + b as f64) / 2.0` |
| `()` | precedence grouping; also the unit type, also a parameter list | 03 | `(a + b) * c` |
| `== != < <= > >=` | comparison, producing a `bool` | 05 | `n % 2 == 0` |
| `&& \|\| !` | boolean and, or, not | 05 | `age >= 16 && has_licence` |
| `,` | separates parameters, tuple fields, match arms | 05 | `(score: u32, bonus: u32)` |

## Borrowing: the two symbols that define Rust

| Symbol | Read as | First taught | Example |
|---|---|---|---|
| `&` | borrow a reference — read it, do not take it | 04 | `fn shout(text: &str) -> String` |
| `&mut` | borrow, with permission to change | 13 | `fn rename(&mut self, to: &str)` |
| `&[T]` | borrow a slice — a `Vec` or an array, without copying | 07 | `fn min_of(values: &[i32]) -> Option<i32>` |
| `&self` / `&mut self` | borrow the object for this method call | 13 | `pub fn describe(&self) -> String` |
| `*` | dereference: take the value the reference points at | 07 | `values.iter().position(\|v\| *v == needle)` |

Why `&` and not a copy: every function in this course that takes `&str` rather than
`String`, or `&[i32]` rather than `Vec<i32>`, is doing it for the same reason — the
caller keeps their value. Section 04 explains it with text; section 07 shows it with a
vector; section 13 makes it unavoidable with `&mut self`.

## Types and paths

| Symbol | Read as | First taught | Example |
|---|---|---|---|
| `->` | returns | 11 | `pub fn half_of(n: i64) -> Option<i64>` |
| `::` | path into a type, module, or enum variant | 06 | `Shake::Vanilla`, `String::from` |
| `<T>` | a type parameter, "of any type T" | 07 | `Vec<i32>`, `HashMap<String, usize>` |
| `Option<T>` | a value that may be absent | 04 | `fn position_of(...) -> Option<usize>` |
| `Result<T, E>` | a value that may be an error | 04 | `fn insert(&mut self, row: &[&str]) -> Result<usize, String>` |
| `()` | the unit type: "returns nothing" | 11 | `pub fn log_only(message: &str)` |
| `!` | the never type: this function does not return | 11 | `pub fn abort_forever() -> !` |
| `Self` | the type this `impl` block is for | 13 | `pub fn new(name: &str) -> Self` |
| `'a` | a lifetime; `'x'` in single quotes is a `char` | 04 | `&'static str` in `classify(score: u32) -> &'static str` |
| `_` | deliberately unused, or "infer this" | 11 | `map(\|_\| rng.roll_die(sides))` |

## Control flow and patterns

| Symbol | Read as | First taught | Example |
|---|---|---|---|
| `=>` | this pattern produces this result, in a `match` | 06 | `Shake::Vanilla => 350,` |
| `..` | a range, end exclusive | 09 | `for i in 1..n` |
| `..=` | a range, end inclusive | 09 | `for i in 1..=n` |
| `\|x\|` | a closure: the parameters, between two pipes | 09 | `.filter(\|x\| x % 2 == 0)` |
| `\|\|` | a closure with no parameters; also boolean or | 09 | `.scan(0i64, \|acc, v\| { *acc += v; Some(*acc) })` |
| `..` in a pattern | the remaining fields, ignored | 06 | `Payment::Card { cents, .. } => cents as i64,` |

## Macros, which are not functions

| Symbol | Read as | First taught | Example |
|---|---|---|---|
| `println!(...)` | the `!` marks a macro, not a call | 01 | `println!("Hello, {trimmed}.")` |
| `format!(...)` | a macro returning a `String` | 04 | `format!("{text}{text}")` |
| `{name}` | inline format argument, a 2021-edition shorthand | 04 | `format!("Hello, {trimmed}.")` |
| `{:?}` / `{:#?}` | debug formatting of a value | 06 | `println!("{payment:?}")` |
| `#[...]` | an attribute: `#[derive]`, `#[test]`, `#[cfg]` | 06 | `#[derive(Debug, Clone, Copy)]` |

## The `?` operator

| Symbol | Read as | First taught | Example |
|---|---|---|---|
| `?` | on `Option`: stop and return `None`; on `Result`: stop and return the error | 04 | `let a: i64 = a.trim().parse().ok()?;` |

Section 04 uses it on `Option` (parsing text), section 14 uses it five times on the URL
parser — the pattern `split_once("://")?` returning early is what makes those functions
read as a single sentence instead of nested `if let`s.

## Shape of a function, in one place

```
pub  fn  describe_dog  (name: &str, age: u8)  ->  String  {  format!(...)  }
│    │   │              │                     │   │       │  │
│    │   │              │                     │   │       │  └ expression: the returned value
│    │   │              │                     │   │       └ body
│    │   │              │                     │   └ return type
│    │   │              │                     └ "returns"
│    │   │              └ parameters: pattern : type
│    │   └ name
│    └ keyword
└ visibility
```

Run `cargo run --example 11_anatomy_of_a_function` to see the same dissection printed,
alongside the associated-function, method, closure and never-returning forms.
