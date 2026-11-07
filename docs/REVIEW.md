# Review wanted

This course teaches beginning programming in Rust. It is correct in the sense that 105 tests
pass and the code compiles; it is **not** yet correct in the sense that anyone has checked the
teaching. Three reviews would change that, and none takes longer than half an hour.

Pick one. Open an issue titled `review: <task>` with what you found, or mail Kevin directly.
A finding can be one sentence. **"This sentence would mislead a beginner because…"** is the most
valuable thing you can send.

## Task A — Rust accuracy, §01–11 (30 minutes)

Read the lesson text, not the code: `src/section_01_comments.rs` … `src/section_11_functions.rs`.
Attack these five claims specifically. Each is a place where I know I simplified:

1. §17 — *"Every value has exactly one owner."* True for the cases taught; wrong once you have
   `Rc`, `Arc`, or shared references. **Would you say this to a beginner, and with what caveat?**
2. §15 — *"a working HTTP/1.0 server and client."* It reads headers only and ignores request
   bodies. What would you call it instead?
3. §18 — the `unwrap`/`expect` policy: panics for bugs, `Result` for the world. Is that a rule
   you would give a beginner, or does it teach a habit they must unlearn?
4. §17 — *"at most one `&mut`, any number of `&`."* Is the wording accurate as stated, or does
   it need the "no live `&` while a `&mut` exists" qualifier spelled out?
5. §04 — `&str` borrowed vs `String` owned, introduced in lesson four. Too early? Too late?

If you have a stronger objection than any of these, ignore the list and send that.

## Task B — beginner read, §01–03 (20 minutes)

Read as a learner, not as a reviewer. `cargo run --example 01_comments`,
`cargo run --example 02_variables`, `cargo run --example 03_operators`, and the matching
`problems/section_0N_parsons.md` files.

- Where did you have to re-read a sentence? Quote it.
- Was the Parsons problem solvable from the lesson alone, or did it need knowledge the lesson
  did not give?
- Did anything stay abstract because there was no picture or no example?

The useful answer is a location and a feeling, not a rewrite.

## Task C — appeal, one video (10 minutes)

Watch `section_02.mp4` from
[`rust-lessons-releases`](https://github.com/kevinelong/rust-lessons-releases) — the shortest
one that still has real content.

- At what point did you stop paying attention, if you did?
- Would you watch the next one? What would make you not?
- Is the narration intelligible at 1× and at 1.5×?

## What counts as a finding

Two examples from this project's own history, because "found a problem" is vague:

* I cited a paper as *"Aalto/ACM, 2026"*; it is **ITiCSE-WGR 2022**. A wrong year is a finding.
* I wrote that the literature gives *"substantial evidence that Parsons problems … outperform
  writing-from-scratch"*; I had read only the front matter. An overstated claim is a finding.

Both were corrected in `docs/research/2026-11-06.md`. Nothing is too small if it would make a
reader believe something untrue.

## What this does not ask of you

No NDA, no obligation, no "review the whole repo", no expectation of code changes. One task, one
pass. If you want credit, say how you want to be named and I will add a `CONTRIBUTORS.md`.

## Defaults, stated so you can disagree with them

* The course targets **complete beginners**; expertise reversal (Kalyuga et al. 2003) means the
  scaffolding may be *harmful* to experienced programmers. If you already program, your best
  contribution is Task A, not B.
* Sources and their read-depth are in [`docs/reading.md`](reading.md) — nothing is marked
  `read` yet, and you are welcome to call that out.
* The course claims correctness and reproducibility. It does **not** claim that anyone learns,
  because nothing here measures that.
