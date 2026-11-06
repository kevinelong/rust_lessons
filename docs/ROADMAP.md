# Roadmap

Derived from `docs/research/2026-11-06.md` and the state in `docs/STATE.md`. Each phase
names an observable result, so "done" is not a matter of opinion.

## Where the course stands (v0.8.0)

| Have | Evidence |
|---|---|
| 20 lessons, 85–88 tests, drift guards | `cargo test` green; a section that loses its prose fails the build |
| 261 slides, 20 narrated videos (26:59) | published as release assets with SHA-256 |
| 20 notebooks that execute against the real Rust kernel | 20/20 clean, outputs inspected |
| six diagrams, embedded in slides and video | verified in a rendered frame |

| Missing | Why it matters |
|---|---|
| **any practice artefact** | nothing asks the learner to write or repair code; the evidence says that is the learning |
| subgoal labels in prose | Margulieux's replicated result: labels improve transfer |
| compiler-message literacy | CMU's study puts Rust learners' top barrier here |
| any measurement | "88 tests" measures code, not comprehension |
| a book-shaped rendering | readers who want linear text get a README and rustdoc |

## Phase 1 — practice artefacts (highest value, cheapest)

Goal: a learner can practise every section without an instructor, and the artefact is
generated rather than authored.

1. ~~Parsons problems, generated~~ — **done in 0.9.0.** `cargo run --bin parsons` writes
   `problems/section_NN.md`: a real function from the section's module (4–16 lines,
   signature included), deterministically shuffled, canonical order collapsed. 20/20
   sections covered. Tests prove the multiset is preserved, the same seed reproduces the
   same scramble, and the canonical first line is never left in place. *Note:* the first
   attempt drew from fenced code in the lesson prose and covered only 1 of 20 sections —
   the code lives in `src/`, not in the prose.
2. **Faded variants** — blank ~30 % of lines, keeping structure. *Done when* the blanks are
   chosen by rule (not randomly) and the file states which subgoal each blank tests.
3. **Retrieval questions** — five per section, drawn from the lesson's own statements, with
   answers in a sibling block. *Done when* a learner can score themselves per section.

## Phase 2 — comprehension support

4. **Subgoal labels** in sections 07, 08, 11, 16 — the procedural ones. *Done when* each
   procedure's steps are named in both the prose and the example file.
5. **Section 21: reading compiler errors** — real transcripts, five classic errors, each with
   the message, the misreading, and the fix. *Done when* the section's example compiles and
   the error texts are captured from real runs, not paraphrased.
6. **Track B diagrams for ownership** — extend the four existing figures to cover move in a
   loop and reborrow, where RustLIVE found visualisation helps most.

## Phase 3 — delivery

7. **mdbook edition** — the same module prose as a browsable book, with `mdbook test`
   verifying code blocks. *Done when* the book builds and its tests run in CI.
8. ~~CI~~ — **added**, `.github/workflows/ci.yml`: build all targets, test, `cargo doc` with
   `-D warnings`, and the curriculum drift guards, on every push. *Hardened when* a red test
   has actually blocked a merge.
9. **Release automation** — regenerate videos and attach them on a tag rather than by hand.

## Phase 4 — measurement

10. **A scoring surface.** Parsons + retrieval artefacts are scoreable; a section ends with
    "you can now do X" and a task that produces a number. *Done when* a stranger's score on
    section 01 can be recorded without an instructor present.
11. **Then, and only then**, a claim about learning — with the caveat that n = 1 is not a study.

## Standing habits

* **Daily research note** in `docs/research/YYYY-MM-DD.md`, following the four-part shape:
  what was read, what it implies, what was decided, what is still unknown.
* **No claim without a measurement.** The failure mode this repo has already slipped into
  once is describing intent as result.
* **One source per fact.** Every derived artefact (slides, video, notebook, problems) is
  regenerated from the lesson module; nothing is edited twice.
