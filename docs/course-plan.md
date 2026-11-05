# Course plan — ten sessions

The sixteen sections of `rust_lessons`, laid out the way the earlier cohorts were
scheduled: the Python deck's 85 slides in ten two-hour sessions, with the exercises
that belong to each. Sections are the unit of progress; sessions are the unit of
class.

Each session ends with something that runs. If a session ends without a program the
student can execute and change, it ran long.

| Session | Sections | Slides | In class | Homework |
|---|---|---|---|---|
| 1 | 01–03 | 2–6 | What a program is; names, comments, values; arithmetic and precedence | Install Rust; `cargo run --example 03_operators`; change the temperature table to include body temperature |
| 2 | 04–05 | 9–22 | Text vs numbers; conversion that can fail; true/false and branching | `parse_sum` exercise: accept floats; classify grades without `if`/`else` (use `match`) |
| 3 | 06 | 23–25 | Enums: a value from a fixed set; `match` must be exhaustive | Add a fourth `Shake`; extend `Payment` with a `Gift` variant and price it |
| 4 | 07–08 | 26–34 | Lists and dictionaries → `Vec` and `HashMap` | `word_counts` on a text file; report the top three words |
| 5 | 09–10 | 35–44 | Ranges and laziness; `for`, `while`, `loop` | FizzBuzz to 100; Collatz for the longest chain under 1,000 |
| 6 | 11 | 49–58 | Functions, arguments, returns, D.R.Y.; the six exercises | `chart_average` with vertical bars; a `roll_d20()` with advantage |
| 7 | 12 | 60–62 | Capstone: guess the number, interactive | Replace the linear reply with a hint range; count attempts by difficulty |
| 8 | 13 | 63–67 | Structs, methods, traits — objects without classes | A `Deck` of `Card`s; a `Speak` implementor of your own |
| 9 | 14–15 | 68–76 | IP, DNS, URLs; clients and servers; JSON without a library | Serve `/echo`; a client that prints headers |
| 10 | 16 | 79–84 | Tables, DDL, CRUD — the last vocabulary needed | Build a `library` table; write the four SQL statements it corresponds to |

## Grading without a rubric

Every exercise in the course already has tests. The rule used in the earlier cohorts
applies here with less ambiguity: **`cargo test` passing is the floor, not the
grade**. A submission is complete when the new behaviour has a test of its own, and
the test fails if the behaviour is removed. That forces the student to describe what
they built, which is the last thing the deck asks for and the first thing a reviewer
needs.

## Where this differs from the Python cohort, and why

* **Session 3 is short.** Enums carry `match`, and `match` is where a student's
  confusion either resolves or compounds; that session leaves room for questions
  instead of new material.
* **No `pip install` session.** There is no dependency management to teach: the course
  has no dependencies, and the first external crate is a deliberate later topic
  (`serde`, then `tokio`) once `std` has been exhausted.
* **Networking is real on day nine.** `serve_n` binds a port and answers a socket in a
  test, so students see the request and response they have been told about, rather than
  a diagram of one.
