# rust_lessons

Introduction to computer programming, taught in **Rust**: the same sixteen lessons
as the Python course (`python_lessons/Introduction_to_computer_programming_with_Python.pptx`,
85 slides), the same exercises, rewritten for the language this repository teaches.

No dependencies, no build scripts, no framework. `std` only, so the course runs on a
bare `cargo` install and a student can read every line of what it does.

```
cargo run                     the course map
cargo run -- 7                section 7's lesson text
cargo run --example 07_vectors   section 7's code, running
cargo test                    61 tests, every section
cargo run -- serve            the section 15 server on 127.0.0.1:8765
```

## What is here

| # | Section | Lesson (from the deck) | Slides | Code | Run it |
|---|---|---|---|---|---|
| 01 | Comments and identifiers | source code is read by people; naming rules | 2–4 | `src/section_01_comments.rs` | `cargo run --example 01_comments` |
| 02 | Variables and data types | a labelled container for a value; types | 3, 20 | `src/section_02_variables.rs` | `cargo run --example 02_variables` |
| 03 | Operators and expressions | `+ - * /`, order of operations | 5–6 | `src/section_03_operators.rs` | `cargo run --example 03_operators` |
| 04 | Strings and conversion | text, numbers, converting between them | 11–16 | `src/section_04_strings.rs` | `cargo run --example 04_strings` |
| 05 | Booleans and conditionals | comparison, branching on true/false | 19, 45–47 | `src/section_05_conditionals.rs` | `cargo run --example 05_conditionals` |
| 06 | Enums | a type whose value is one of a named set | 23 | `src/section_06_enums.rs` | `cargo run --example 06_enums` |
| 07 | Vectors | sequences: create, index, push, iterate | 26–30 | `src/section_07_vectors.rs` | `cargo run --example 07_vectors` |
| 08 | Maps | looking a value up by another value | 33–34 | `src/section_08_maps.rs` | `cargo run --example 08_maps` |
| 09 | Ranges and iterators | sequences generated as you walk them | 35 | `src/section_09_iterators.rs` | `cargo run --example 09_iterators` |
| 10 | Loops | `for`, `while`, repeat until done | 38–44 | `src/section_10_loops.rs` | `cargo run --example 10_loops` |
| 11 | Functions | D.R.Y., arguments, returns; **anatomy of a function**; `min`, `max`, dice, `reverse_me` | 49–58 | `src/section_11_functions.rs` | `cargo run --example 11_functions`, `cargo run --example 11_anatomy_of_a_function` |
| 12 | Capstone | guess the number — the first useful program | 60–62 | `src/section_12_capstone.rs` | `cargo run --example 12_capstone` |
| 13 | Structs, methods, traits | objects as autonomous cells that talk | 63–67 | `src/section_13_structs.rs` | `cargo run --example 13_structs` |
| 14 | Networks, IP, DNS, URLs | an address, a directory, a name for a thing | 68–71 | `src/section_14_networking.rs` | `cargo run --example 14_networking` |
| 15 | Clients and servers | many consumers, one responder | 73–76 | `src/section_15_client_server.rs` | `cargo run --example 15_client_server` |
| 16 | Tables and SQL | DDL, and CRUD — create, read, update, delete | 79–84 | `src/section_16_databases.rs` | `cargo run --example 16_databases` |

`docs/course-plan.md` lays the same sixteen sections out as a ten-session course with
timings, the way the earlier cohorts were scheduled.

## Structure, and why

The course keeps the shape that worked in the earlier language courses — one numbered
document per lesson, a glossary, and problems paired with solutions — adapted to a
language that has to compile:

```
src/section_NN_topic.rs   the lesson as documentation, plus the functions it teaches,
                          plus that section's tests (#[cfg(test)] mod tests)
examples/NN_topic.rs      the same lesson as a program you can run and change
tests/curriculum.rs       the invariants of the course as a whole
docs/course-plan.md       the ten-session plan
docs/symbols.md           every symbol in the course: what it means, which section teaches it
```

Three deliberate choices:

* **Tests are the acceptance criteria.** The deck's exercises (`min`, `max`, `roll`,
  `get_average_roll`, `trackAverage`, `chart_average`, `reverse_me`) each carry the
  tests that define "done", so a student and a grader agree without a rubric.
* **Every example runs.** `cargo run --example 15_client_server` starts a real server on
  a kernel-assigned port, sends four real requests through it, and exits — so the
  networking lessons are executable rather than described.
* **Symbols are taught where they are first needed, and indexed.** `&` arrives in
  section 04 with `&str`, `|x|` in section 09 with closures, `&mut self` in section 13
  with methods; `docs/symbols.md` is the single table that says which section owns which
  symbol, so a student can look one up instead of hunting.
* **Teaching text lives above the code it explains.** Each module's `//!` header is the
  lesson, written to be read aloud, and `cargo doc --open` renders it as notes.

## Rust-specific lessons the Python course could not teach

These are added where the language makes them unavoidable, not as extra topics:

| Idea | Where | Why it appears here |
|---|---|---|
| immutability by default, `mut` | 02 | the deck already says a variable is a labelled container; Rust asks whether the label may be rewritten |
| type inference, then annotations | 02, 04 | `let x = 3` has a type even when unwritten |
| integer and float division | 02, 03 | `(1 + 2) / 2` is 1; the deck's `average` becomes a lesson in casting |
| `&str` borrowed vs `String` owned | 04 | text conversion is where Python's single `str` hides a real distinction |
| `Option` and `Result` instead of exceptions | 04, 06, 12 | `parse` can fail, so the failure is in the type |
| exhaustive `match` | 06, 12 | a missing enum case is a compile error, not a runtime surprise |
| traits instead of class hierarchies | 13 | Alan Kay's "autonomous cells" without inheritance |
| borrow-checked subprocesses-free networking | 15 | threads and sockets with no unsafe and no dependencies |

## Versioning

One constant, `VERSION` in `src/version.rs`, is the only place the version is written;
`tests/curriculum.rs` asserts that `Cargo.toml` agrees with it. The rule is
major.minor.patch: patch for a bug fix, minor for a feature, major only for a published
release. This repository started at **0.1.1**; the anatomy-of-a-function lesson and the symbol index made it **0.2.0** (minor: a feature, patch reset to 0).

## Licence

FSL-1.1-MIT — see [LICENSE.md](LICENSE.md). The curriculum it mirrors (the Python deck,
the Java and JavaScript cohorts, the PDX Code Guild material) is not included and
remains under its own terms.
