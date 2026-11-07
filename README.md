# rust_lessons

> Picking this up cold? Read [CONTINUE-HERE.md](CONTINUE-HERE.md) first, then
> [docs/STATE.md](docs/STATE.md).

Introduction to computer programming, taught in **Rust**: the same sixteen lessons
as the Python course (`python_lessons/Introduction_to_computer_programming_with_Python.pptx`,
85 slides), the same exercises, rewritten for the language this repository teaches.

No dependencies, no build scripts, no framework. `std` only, so the course runs on a
bare `cargo` install and a student can read every line of what it does.

```
cargo run                     the course map
cargo run -- 7                section 7's lesson text
cargo run --example 07_vectors   section 7's code, running
cargo test                    139 tests; one long one is #[ignore]d
cargo run --bin narrate -- 1   generate section 1's slides, narration text and mux scripts
cargo run --bin notebook       generate all twenty Jupyter notebooks
cargo run --bin parsons        generate the Parsons and faded problems (problems/)
cargo run --bin quiz           generate the retrieval checks (problems/)
cargo run --bin score          score a session: correctness, time, retrieval vs recognition
cargo run --bin book           write the mdBook chapters (book/src/)
cargo run --bin captions       write SRT subtitles for every video (out/video/)
tools/release-videos.ps1       regenerate every video and publish a release (see docs/STATE.md)
mdbook serve book              read the course as a book (after: mdbook build book)
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
| 07 | Vectors | sequences: create, index, push, iterate; **subgoals**, with a labelled worked example | 26–30 | `src/section_07_vectors.rs` | `cargo run --example 07_vectors` |
| 08 | Maps | looking a value up by another value; **subgoals**, with a labelled worked example | 33–34 | `src/section_08_maps.rs` | `cargo run --example 08_maps` |
| 09 | Ranges and iterators | sequences generated as you walk them | 35 | `src/section_09_iterators.rs` | `cargo run --example 09_iterators` |
| 10 | Loops | `for`, `while`, repeat until done | 38–44 | `src/section_10_loops.rs` | `cargo run --example 10_loops` |
| 11 | Functions | D.R.Y., arguments, returns; **anatomy of a function**; **subgoals**, with a labelled worked example; `min`, `max`, dice, `reverse_me` | 49–58 | `src/section_11_functions.rs` | `cargo run --example 11_functions`, `cargo run --example 11_anatomy_of_a_function` |
| 12 | Capstone | guess the number — the first useful program | 60–62 | `src/section_12_capstone.rs` | `cargo run --example 12_capstone` |
| 13 | Structs, methods, traits | objects as autonomous cells that talk | 63–67 | `src/section_13_structs.rs` | `cargo run --example 13_structs` |
| 14 | Networks, IP, DNS, URLs | an address, a directory, a name for a thing | 68–71 | `src/section_14_networking.rs` | `cargo run --example 14_networking` |
| 15 | Clients and servers | many consumers, one responder | 73–76 | `src/section_15_client_server.rs` | `cargo run --example 15_client_server` |
| 16 | Tables and SQL | DDL, and CRUD — create, read, update, delete; **subgoals**, with a labelled worked example | 79–84 | `src/section_16_databases.rs` | `cargo run --example 16_databases` |

**Track B — required by Rust, absent from the deck** (the deck has no slide for any of
these, and `python_lessons` teaches them in files rather than slides). Section 21's five
error transcripts were **captured from `rustc`**, not written from memory: see
[docs/compiler-errors.md](docs/compiler-errors.md).

| # | Section | Why it is here | Code | Run it |
|---|---|---|---|---|
| 17 | Ownership and moves | `&` is taught in 04; without this the borrow has no reason | `src/section_17_ownership.rs` | `cargo run --example 17_ownership` |
| 18 | Errors | `assert_try_except.py` in the Python course; two levels, `panic` and `Result` | `src/section_18_errors.rs` | `cargo run --example 18_errors` |
| 19 | The keyboard and the disk | `interactive.py` and `09_file_input_and_output.py` in the Python course | `src/section_19_input_output.rs` | `cargo run --example 19_input_output` |
| 20 | Testing | the Python course shipped `problems/` pairs; the JS cohorts a whole `tdd_*` family | `src/section_20_testing.rs` | `cargo run --example 20_testing` |
| 21 | Reading compiler errors | the Ruby track had *reading error messages*; the deck had none, and CMU's study puts compiler messages among Rust's top barriers | `src/section_21_compiler_errors.rs` | `cargo run --example 21_compiler_errors` |

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
docs/diagrams/            six diagrams: four SVG, two Mermaid
docs/video.md             the format ladder and the narrated-video pipeline
docs/STATE.md             current state, decisions, defects found, open items
docs/ROADMAP.md           the plan, phased, each phase with an observable result
docs/REVIEW.md            three bounded review tasks — start here if you want to review
docs/reading.md           further reading per section: one source to go deeper, one to disagree
docs/research/            daily research notes: what was read, what it implies
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

## Diagrams

Six concepts are structural rather than sequential, so each has a drawing: the test for
"does this earn a diagram" was whether the idea is about parts in relationship (or a
count rule over those parts) and cannot be written as one line of code or a two-column
table.

Two of the six are *sequences* rather than structures, and those are Mermaid — a
`flowchart` and a `sequenceDiagram` — so they follow the reader's theme and survive in a
text diff. The other four need arrowheads landing on specific boxes and cells, which
Mermaid cannot place, so they are SVG. Mermaid strips angle brackets from labels, which
is why the pipeline's consumer reads `.collect()`; the turbofish is in the lesson.

| Diagram | Shows | Used by |
|---|---|---|
| `docs/diagrams/ownership.svg` | move, borrow and clone side by side, with the old name dead after a move | section 17 |
| `docs/diagrams/borrow-rules.svg` | many `&`, or exactly one `&mut`, never both | sections 04, 17 |
| `docs/diagrams/string-stack-heap.svg` | the three words a `String` keeps on the stack, the bytes it points at | section 04 |
| `docs/diagrams/flow.md#iterator-laziness` | **Mermaid** `flowchart LR`: lazy stages, and the consuming stage that finally runs them | section 09 |
| `docs/diagrams/flow.md#-propagation` | **Mermaid** `sequenceDiagram`: an error leaving three frames early, and what the `?` skipped | section 18 |
| `docs/diagrams/match-exhaustiveness.svg` | why an arm per variant beats `_ =>`, and what E0004 looks like | section 06 |

Diagrams that were considered and rejected: CRUD↔SQL (a 4×2 table, already written),
operators and comparison tables (flat lookups), client/server (near-tautological once
section 15's socket code is on the page), and the guess-the-number state machine (three
states, easier to read as prose).

## Reviewing it

[`docs/REVIEW.md`](docs/REVIEW.md) is the review packet: three tasks of 20–30 minutes each
(Rust accuracy on §01–11, a beginner read of §01–03, an appeal pass on one video), each with the
specific claims to attack and two worked examples of what counts as a finding. The code is
verified; the teaching is not.

## Where to go next

[docs/reading.md](docs/reading.md) gives every section a primary source and an **alternate
take** — the same material argued differently — with the depth each was read to (`named`,
`skimmed`, `read`). It exists because a citation that only proves a point is half a citation:
the learner who wants to go further, or to disagree, needs somewhere to go.

A `read` there is checkable rather than a boast: the entry carries an evidence tag, and the
block it points at states the pages actually looked at and quotes at least one line out of the
source. `cargo test` fails if an entry marked `read` has no such quote — the rule is that a
reviewer can always ask "show me the sentence" and get one.

The strongest entry there is the **expertise reversal effect**: scaffolding that helps novices
can *harm* learners who already program. That is the honest case against this course for
anyone who is not a beginner.

## As a book

`cargo run --bin book && mdbook serve book` renders the same prose as an mdBook: one
chapter per section (23 chapters in `book/src/`, generated and committed). `mdbook test`
compiles the book's `rust` block — and that check was verified not to be vacuous: breaking
the block fails the test with `E0277`. The chapters are
generated from `src/section_*.rs`, so editing a lesson updates the book, the slides, the
notebook, the practice problems and the video script.

## Practice

`problems/` holds three generated practice artefacts per section — 63 files, none authored
by hand:

| File | Exercise | Generator |
|---|---|---|
| `section_NN_parsons.md` | the lines of a real function, out of order — reassemble them | `cargo run --bin parsons` |
| `section_NN_faded.md` | structure given, **decision points blanked** (`let`/`if`/`for`/`match`) | `cargo run --bin parsons` |
| `section_NN_quiz.md` | retrieval: four to five sentences with one term removed, answers collapsed | `cargo run --bin quiz` |

The retrieval check has a floor: `cargo test` asserts every section yields at least four
scorable items (101 items over 21 sections: seventeen at five, four at four), because a
section with three cannot be scored — one wrong answer would move its mark by a third.

A session is data before it is a score. `fixtures/session-2026-11-07-synthetic.json` is one
recorded session — answers, timings, and one item answered wrong and then right — read by
`rust_lessons::session` and asserted number by number in `tests/session_fixture.rs`, with no
window and no network. It is synthetic: written, not sat, so it can test a scoring rule but
can never be evidence that anyone learned anything.

`cargo run --bin score` reads that session and prints phase 4's table: correctness, time to
answer, and retrieval versus recognition for every section, plus the whole session. A
retried item shows as more tries than items, and a kind of item a section never asked prints
`-` rather than a silent zero:

```text
session synthetic-2026-11-07-a | learner: synthetic-fixture | synthetic - written, not sat

section  items  tries  correct  accuracy   mean s  total s  retrieval         recognition
01           3      4        2     50.0%     13.4     53.7  2/3 (66.7%)       0/1 (0.0%)
02           2      2        1     50.0%      4.7      9.4  1/2 (50.0%)       -
03           2      2        2    100.0%     19.6     39.3  1/1 (100.0%)      1/1 (100.0%)
overall      7      8        5     62.5%     12.8    102.4  4/6 (66.7%)       1/2 (50.0%)

last row is the whole session; '-' means no item of that kind was asked.
```

Point it at any session the schema accepts: `cargo run --bin score -- path/to/session.json`.
Every number above is asserted in `tests/session_scoring.rs`, and changing one recorded
answer changes the table — it is computed from the answers, not stored.

Why Parsons: it removes the blank-page cost while keeping the reasoning, and the literature
puts it above writing-from-scratch for early practice. Sources and what they imply are in
[docs/research/2026-11-06.md](docs/research/2026-11-06.md); the plan it drives is
[docs/ROADMAP.md](docs/ROADMAP.md).

## Formats

One source, four renderings — lesson prose, slides, notebooks and narrated video, all
generated. `cargo run --bin narrate -- N` turns a
section's prose into beats, one HTML slide per beat, the narration text, and two scripts
that synthesise the audio (Kokoro-82M, voice `af_heart`; or Windows SAPI as a fallback)
and mux the MP4. Timing is narration-driven, so the video
re-derives itself when the prose changes. Details, verified output and publishing notes:
[docs/video.md](docs/video.md).

**Watch them:** every video ships an **SRT track** — one cue per narrated beat, timed from the
same durations the mux uses, so captions cannot drift from the picture or from the prose. The
generated videos are published as release assets in
[`rust-lessons-releases`](https://github.com/kevinelong/rust-lessons-releases/releases) —
**all 21 sections, 29:43**, 1920×1080 with narration, verified by SHA-256. They are
build output, so they live there and not here.

A test guards the derivation: `every_section_has_prose_to_narrate` fails the build if a
section stops producing spoken beats, because that would be a silent video rather than an
error.

A second test guards the pedagogy's own precondition. Margulieux & Catrambone (2016,
[docs/reading.md](docs/reading.md) [E3]) found that subgoal labels in text helped only when a
worked example carried the same labels — the labelled text on its own did not. So
`subgoals_travel_with_a_worked_example` fails if a section names subgoals and its
`# Worked example` does not repeat those names, or if the example quotes no code or states no
output: repeating a label without working the problem would be a second copy of the block.

## Versioning

One constant, `VERSION` in `src/version.rs`, is the only place the version is written;
`tests/curriculum.rs` asserts that `Cargo.toml` agrees with it. The rule is
major.minor.patch: patch for a bug fix, minor for a feature, major only for a published
release. This repository started at **0.1.1**; the anatomy-of-a-function lesson and symbol index made it **0.2.0**, Track B plus the diagrams made it **0.3.0**, and the video generator made it **0.4.0**, voice selection **0.5.0**, and covering all twenty sections **0.6.0**, the notebook tier **0.7.0**, diagrams embedded in slides and video **0.8.0**, practice artefacts **0.10.0**, compiler-error literacy **0.11.0**, subgoal labels **0.12.0**, the mdBook edition **0.13.0**, release automation **0.14.0**, the published video set **0.15.0**, a retrieval floor every section meets **0.16.0**, a recorded session as data **0.17.0**, scoring it **0.18.0**, the checked reading register **0.19.0**, and subgoal labels paired with the worked examples that justify them **0.20.0** — minor bumps for features, patch reset to 0 each time.

## Licence

FSL-1.1-MIT — see [LICENSE.md](LICENSE.md). The curriculum it mirrors (the Python deck,
the Java and JavaScript cohorts, the PDX Code Guild material) is not included and
remains under its own terms.
