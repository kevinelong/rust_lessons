# Further reading

Citations do two jobs: they show where a claim came from, and they let a reader go past the
course — deeper on the same view, or sideways to a different one. This page is the second job.

Each section lists **one primary source** (read this to go deeper) and **one alternate take**
(read this to disagree with me). Where I could not verify a stable URL I name the source and say
where to find it rather than inventing a link.

Status column, honestly: **read** = I have read it at the depth the course relies on;
**skimmed** = front matter or abstract only; **named** = known to exist, not read.

## By section

| § | Primary | Alternate take |
|---|---|---|
| 01 Comments and identifiers | Rust Book, [ch03-01 variables and mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html) — **skimmed** | Rust API guidelines on [naming](https://rust-lang.github.io/api-guidelines/naming.html), for the conventions the book only hints at — **named** |
| 02 Variables and data types | Rust Book, [ch03-02 data types](https://doc.rust-lang.org/book/ch03-02-data-types.html) — **skimmed** | The Reference on [type inference and integer overflow](https://doc.rust-lang.org/reference/) — **named**: the normative text behind the prose |
| 03 Operators and expressions | Rust Book, [ch03-03 functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html) on expressions vs statements — **skimmed** | *Rust by Example* on [expressions](https://doc.rust-lang.org/rust-by-example/) — **named**: same idea, more examples, no argument |
| 04 Strings and conversion | Rust Book, [ch08-02 strings](https://doc.rust-lang.org/book/ch08-02-strings.html) — **skimmed** | The Reference on [`str` and UTF-8](https://doc.rust-lang.org/reference/) — **named**: why byte length and char count differ |
| 05 Booleans and conditionals | Rust Book, [ch03-05 control flow](https://doc.rust-lang.org/book/ch03-05-control-flow.html) — **skimmed** | The Reference on [`if` as an expression](https://doc.rust-lang.org/reference/expressions/if-expr.html) — **named** |
| 06 Enums | Rust Book, [ch06-00 enums and pattern matching](https://doc.rust-lang.org/book/ch06-00-enums.html) — **skimmed** | The Rustonomicon on [enums and layout](https://doc.rust-lang.org/nomicon/) — **named**: what a variant costs, which the course never asks |
| 07 Vectors | Rust Book, [ch08-01 vectors](https://doc.rust-lang.org/book/ch08-01-vectors.html) — **skimmed** | clippy's [`len_zero` and slicing lints](https://rust-lang.github.io/rust-clippy/) — **named**: the same code, reviewed by a tool |
| 08 Maps | Rust Book, [ch08-03 hash maps](https://doc.rust-lang.org/book/ch08-03-hash-maps.html) — **skimmed** | `std::collections` docs on [SipHash and DoS resistance](https://doc.rust-lang.org/std/collections/) — **named**: why `.entry()` exists at all |
| 09 Ranges and iterators | Rust Book, [ch13-02 iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html) — **skimmed** | *Programming Rust* (Blandy, Orendorff, Tindall) on iterator adaptors — **named**: the book that explains the costs |
| 10 Loops | Rust Book, [ch03-05 control flow](https://doc.rust-lang.org/book/ch03-05-control-flow.html) — **skimmed** | The Reference on [`loop` as a value-producing expression](https://doc.rust-lang.org/reference/expressions/loop-expr.html) — **named** |
| 11 Functions | Rust Book, [ch03-03 how functions work](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html) — **skimmed** | Margulieux & Catrambone on [subgoal labels in expository text](https://www.sciencedirect.com/science/article/abs/pii/S095947521530044X) — **skimmed**: the evidence behind the Subgoals block |
| 12 Capstone: guess the number | Rust Book, [ch02 programming a guessing game](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html) — **skimmed** | *Programming Rust*'s treatment of `Ordering` and seeding — **named**: the same program without the training wheels |
| 13 Structs, methods and traits | Rust Book, [ch05](https://doc.rust-lang.org/book/ch05-00-structs.html) and [ch10-02 traits](https://doc.rust-lang.org/book/ch10-02-traits.html) — **skimmed** | Rust Design Patterns (rust-unofficial) on *newtype* and *RAII guards* — **named**: what people do instead of inheritance |
| 14 Networks, IP, DNS and URLs | *Beej's Guide to Network Programming* — **named**: the classic explanation of the stack, in C | RFC 3986 (URI syntax) — **named**: why `parse_url` is a shape check, not a parser |
| 15 Clients and servers | Rust Book, [ch20-01 a single-threaded web server](https://doc.rust-lang.org/book/ch20-01-single-threaded.html) — **skimmed**: the same lesson at more length | RFC 9110 §9 (methods) — **named**: what a "working HTTP/1.0 server" still has to be able to say |
| 16 Tables and SQL | *SQLite* documentation on [CREATE TABLE](https://sqlite.org/lang_createtable.html) — **skimmed** | *Designing Data-Intensive Applications*, ch. 2 (relational vs document models) — **named**: the argument against starting with tables |
| 17 Ownership and moves | Rust Book, [ch04-00 understanding ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html) — **skimmed** | RustLIVE (IEEE FIE 2024), [visualising ownership](https://www.computer.org/csdl/proceedings-article/fie/2024/10893207/24Eno9SKmhW) — **skimmed**: the paper that measures what my diagrams assume |
| 18 Errors | Rust Book, [ch09-00 error handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html) — **skimmed** | The `thiserror`/`anyhow` READMEs — **named**: the ecosystem's answer, and where my custom enum stops being enough |
| 19 Keyboard and disk | `std::io` and `std::fs` documentation — **named** | The Rust Cookbook — **named**: the same tasks done idiomatically |
| 20 Testing | Rust Book, [ch11-00 writing tests](https://doc.rust-lang.org/book/ch11-00-writing-tests.html) — **skimmed** | The *Rust Fuzz Book* — **named**: what testing looks like when the input is hostile |
| 21 Reading compiler errors | Rustc's [error index](https://doc.rust-lang.org/error_codes/error-index.html) — **named**: every `E` number, expanded | The CMU [multimodal study of Rust challenges](https://kilthub.cmu.edu/articles/conference_contribution/A_Multimodal_Study_of_Challenges_Using_Rust/22277326) — **skimmed**: which errors actually stop learners |

## On the teaching design

| Claim the course makes | Primary | Alternate take |
|---|---|---|
| Parsons problems scaffold early practice | Ericson et al., *Parsons Problems and Beyond*, [ITiCSE-WGR 2022](https://doi.org/10.1145/3571785.3574127) — **not read beyond front matter: corrected from a wrong 2026 citation** | Haynes-Magyar, [*Learning to Program != One-Size-Fits-All*](https://arxiv.org/abs/2512.22407), n = 9 perceptions study — **skimmed**: argues the variant matters more than the technique |
| Subgoal labels improve transfer | Margulieux & Catrambone 2016 — **skimmed** | **Expertise reversal effect** (Kalyuga, Ayres, Chandler, Sweller, 2003): scaffolding that helps novices *harms* learners with prior knowledge. The strongest argument against this course's whole approach for anyone who already programs — **named**, and the reason the book says who the course is *not* for |
| Faded Parsons are a reasonable next step | Joentausta & Hellas, [SIGCSE 2018](https://doi.org/10.1145/3159450.3159494) — **skimmed** | PriM (Sentance & Waite): *Predict–run–investigate–modify–make* — **named**: a sequence that front-loads comprehension before construction |
| Visualising memory helps | RustLIVE (FIE 2024) — **skimmed** | Sweller's cognitive load theory on split attention — **named**: a diagram beside text can *add* load if it must be cross-referenced |

## A standing rule

The status column is not decoration. A claim resting on a **named** source is a guess, and the
course says so; a claim resting on a **skimmed** source is one paper-deep. Only the entries marked
**read** would survive a reviewer asking "show me the sentence".

As of 2026-11-06, **none** are marked read. That is the honest state and the next thing to fix.
