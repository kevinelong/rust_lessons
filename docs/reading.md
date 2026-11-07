# Further reading

Citations do two jobs: they show where a claim came from, and they let a reader go past the
course — deeper on the same view, or sideways to a different one. This page is the second job.

Each section lists **one primary source** (read this to go deeper) and **one alternate take**
(read this to disagree with me). Where I could not verify a stable URL I name the source and say
where to find it rather than inventing a link.

Status column, and what each word buys:

* **read** — read at the depth the course relies on, and the note below holds a line copied out
  of it. Carries an evidence tag (`[E1]`) that must resolve to a block in
  [Evidence](#evidence); `tests/curriculum.rs::every_read_source_has_a_quoted_line` fails if it
  does not.
* **skimmed** — front matter or abstract only.
* **named** — known to exist, not read.

A page range plus a quoted sentence is what makes "read" checkable rather than a mood. The five
sources the teaching-design claims actually rest on are the ones audited below; the per-section
rows name sources at the depth they were used, and most were used at title level.

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
| 11 Functions | Rust Book, [ch03-03 how functions work](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html) — **skimmed** | Margulieux & Catrambone 2016 on subgoal labels **in text and worked example together** — **read** [E3]: the evidence behind the Subgoals block, and it only holds when the example carries the same labels |
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
| Parsons problems scaffold early practice | Ericson et al., *Parsons Problems and Beyond*, [ITiCSE-WGR 2022](https://doi.org/10.1145/3571785.3574127) — **read** [E1]: it defines the problem type and reviews the benefit claims without replicating any of them | Haynes-Magyar, [*Learning to Program != One-Size-Fits-All*](https://arxiv.org/abs/2512.22407) — **read** [E2]: n = 9, perceptions, and it argues the *variant* matters more than the technique |
| Subgoal labels improve problem solving and transfer | Margulieux & Catrambone, *Learning and Instruction* 42 (2016) 58–71, [doi:10.1016/j.learninstruc.2015.12.002](https://doi.org/10.1016/j.learninstruc.2015.12.002) — **read** [E3]: labels in **both** text and worked example; a labelled text alone did not independently help | **Expertise reversal effect** (Kalyuga, Ayres, Chandler, Sweller, 2003), [doi:10.1207/s15326985ep3801_4](https://doi.org/10.1207/s15326985ep3801_4) — **skimmed** [E5]: scaffolding that helps novices can *harm* learners with prior knowledge, and is the reason the book says who the course is *not* for |
| Subgoal labels work with younger learners too | Joentausta & Hellas, *Subgoal Labeled Worked Examples in K-3 Education*, [SIGCSE 2018](https://doi.org/10.1145/3159450.3159494) — **skimmed** [E4]: 9–10-year-olds, n = 43, LightBot | Margulieux, Morrison, Decker et al., *Varying effects of subgoal labeled expository text in programming, chemistry, and statistics*, [Instructional Science 2018](https://doi.org/10.1007/s11251-018-9451-7) — **named**: the effect is not uniform across domains |
| Faded Parsons are a reasonable next step | Weinman, Fox & Hearst, *Improving Instruction of Programming Patterns with Faded Parsons Problems*, [CHI 2021](https://doi.org/10.1145/3411764.3445228) — **named**: the paper that introduced the variation the course generates | PriM (Sentance & Waite): *Predict–run–investigate–modify–make* — **named**: a sequence that front-loads comprehension before construction |
| Visualising memory helps | RustLIVE (FIE 2024) — **skimmed** | Sweller's cognitive load theory on split attention — **named**: a diagram beside text can *add* load if it must be cross-referenced |

## Evidence

Each block answers two questions a reviewer is entitled to ask: which pages did you look at, and
can you copy a sentence out of them that says what you claim. Retrieval dates are the day the
copy was fetched; links rot, quotes do not.

### E1 — Ericson, Denny, Prather, Duran, Hellas, Leinonen, Miller, Morrison, Pearce & Rodger (2022)

- Status: read
- Seen: pp. 1–9 of 44 (abstract, §1 introduction, §2 definitions and Table 1 characteristics,
  §2.5 related theories, §3 review method and search) — 2026-11-07, authors' PDF copy
  ([juholeinonen.com](https://juholeinonen.com/assets/pdf/ericson2022parsons.pdf)), bibliographic
  record checked against [doi:10.1145/3571785.3574127](https://doi.org/10.1145/3571785.3574127)
- Claim it supports: what a Parsons problem is, that the literature claims benefits, and that
  those benefits have not been replicated at scale. That is the depth the course relies on, and
  it is all the course may claim.
- Quoted:
  > Parsons problems offer a simpler alternative to writing code by providing scrambled code
  > blocks that must be placed in the correct order to solve a problem. (abstract, p. 1)
  > In the 16 years since their introduction to the computing education community, an expansive
  > body of literature has emerged that documents a range of tools, novel problem variations and
  > makes numerous claims of benefits to learners. (abstract, p. 1)
  > However, there have been no large scale efforts to organise this literature and no large
  > scale attempts at replication to verify these claims. (§1, p. 2)
  > RQ3: What evidence exists for the claimed benefits that Parsons problems offer, and for their
  > limitations, and what is the quality of that evidence? (§3.1, p. 8)
  > The worked example effect decreases and can even reverse as expertise increases [209]. This
  > is called the expertise reversal effect. (§2.5.2, p. 7)
- Does not support: that Parsons problems outperform writing from scratch. The review surveys
  claims and designs studies to test them; it does not report a replication of its own.
- Note: §2.5.2, quoted above, is where this 2022 review hands the expertise-reversal argument to
  **E5** — the two sources are not independent, and the course should not count them twice.

### E2 — Haynes-Magyar (2025), *Learning to Program != "One-Size-Fits-All"*

- Status: read
- Seen: arXiv PDF, abstract and §§1–4.4 (introduction, related work, method, participants, the
  think-aloud results, cognitive-load ratings) — 2026-11-07,
  [arXiv:2512.22407v1](https://arxiv.org/abs/2512.22407) [cs.HC], 26 Dec 2025; §5 onward not
  read
- Claim it supports: the *variation* matters — learners pick faded problems for syntax and
  pseudocode problems for reasoning — and it is the strongest counter-argument to treating
  "Parsons problem" as one intervention.
- Quoted:
  > Retrospective think-aloud interviews were conducted with nine programmers who were given the
  > choice between Faded Parsons and Pseudocode Parsons problems as optional scaffolding toward
  > solving write-code problems. (abstract, p. 1)
  > Learners selectively used Faded Parsons problems for syntax/structure and Pseudocode Parsons
  > problems for high-level reasoning. (abstract, p. 1)
  > Faded Parsons problems were also perceived as a desirable challenge. (abstract, p. 1)
  > Across the eleven problems, the average *within-person* rating (mean of each participant's
  > eleven ratings) was *M* = 3.80 (*SD* = 1.14, *N* = 9), indicating *"rather low mental
  > effort."* (§4.2, p. 8)
- Does not support: an effect on learning. Nine participants, interviewed, on their own
  perceptions; nothing here is an outcome measure, and the paper's "empirical evidence of
  effectiveness" is evidence about scaffolding *use*, not about attainment.
- Note: the arithmetic above is why this entry is `read` and still cannot be cited as "faded
  Parsons work": n = 9 with self-report is a hint, and the register says so.

### E3 — Margulieux & Catrambone (2016)

- Status: read
- Seen: pp. 58–67 of *Learning and Instruction* 42:58–71 — abstract, §1 and §1.1–1.4, §2 overview,
  §3.1–3.2 (Experiment 1 method, n = 120), §4.1–4.3 (results and discussion) — 2026-11-07,
  authors' copy ([sites.gatech.edu](https://sites.gatech.edu/richardcatrambone/files/2020/09/MargulieuxandCatrambone2016.pdf)),
  record checked against [doi:10.1016/j.learninstruc.2015.12.002](https://doi.org/10.1016/j.learninstruc.2015.12.002)
- Claim it supports: subgoal labels in expository text **and** a worked example carry the same
  labels improve solving of novel problems — and only in that combination.
- Quoted:
  > The results show that learners who received subgoal labels in both the text and example are
  > able to solve novel problems better than those who did not. (abstract, p. 58)
  > Closer evaluation showed that there was not a consistent simple effect of text design meaning
  > subgoal labels in the text did not independently improve performance (§4.2, p. 65)
  > First, they demonstrate that for subgoal labeled text to improve performance, it needs to be
  > paired with subgoal labeled examples. (§4.2, p. 65)
  > Participants were 120 students from a mid-sized, urban university who received class credit
  > for participation. (§3.1.1, p. 61)
- Does not support: that labelling the prose is enough on its own. This is the finding that
  bites: the course puts a **Subgoals** block in the lesson text of §07, §08, §11 and §16, and
  the experiment that justifies it paired the labelled text with a labelled *example*. A
  Subgoals block with no labelled example beside it is outside the tested condition.
- Note: the dependent measures are written problem-solving steps, not compilable programs, and
  the domain is App Inventor blocks rather than Rust.

### E4 — Joentausta & Hellas (2018)

- Status: skimmed
- Seen: bibliographic record and abstract only — 2026-11-07,
  [doi:10.1145/3159450.3159494](https://doi.org/10.1145/3159450.3159494) (ACM Digital Library
  refused a full-text fetch: HTTP 403), abstract as indexed by OpenAlex and CrossRef
- Claim it supports: that subgoal-labelled worked examples have been tested with children, which
  is what the "not an artefact of adult expertise" line needs — the test exists.
- Quoted:
  > We conducted a study on the applicability of subgoal labeled worked examples with 9 and
  > 10-year-old pupils (n=43) who were learning the principles of programming using LightBot.
  > (abstract)
- Does not support: anything about *faded Parsons* problems, which this paper does not study —
  it is why the faded-Parsons row above now cites Weinman, Fox & Hearst instead (correction C3).
  Nor does an abstract saying what was studied say what was found.
- Why still skimmed: the result sentence is behind the paywall. Flipping this to `read` would
  mean quoting a title, which is exactly the failure this page exists to prevent.

### E5 — Kalyuga, Ayres, Chandler & Sweller (2003)

- Status: skimmed
- Seen: abstract only — 2026-11-07,
  [doi:10.1207/s15326985ep3801_4](https://doi.org/10.1207/s15326985ep3801_4) (*Educational
  Psychologist* 38(1):23–31); the UNSW repository record for the accepted version states
  "metadata only access", so no full text was reachable
- Claim it supports: expertise reversal exists as a named effect — scaffolding effective for
  novices can lose effectiveness or harm experienced learners.
- Quoted:
  > Instructional techniques that are highly effective with inexperienced learners can lose
  > their effectiveness and even have negative consequences when used with more experienced
  > learners. We call this phenomenon the expertise reversal effect. (abstract)
- Does not support: any specific magnitude, or that *this* course's scaffolding is past the
  reversal point for programmers. That needs the review's body, which was not read.
- Second-hand route: **E1** §2.5.2 summarises the same effect, so the course's argument does not
  hang on the unread article alone — it hangs on two sources that are not independent.

## Corrections

Two errors were found on 2026-11-06 and one more during this audit on 2026-11-07. Each row names
what was wrong, what replaced it, and the evidence for the replacement.

| # | Was | Is | Evidence |
|---|---|---|---|
| C1 | The Parsons review cited as "Aalto/ACM, 2026" | Ericson et al., *Parsons Problems and Beyond: Systematic Literature Review and Empirical Study Designs*, ITiCSE-WGR '22, Dublin, 8–13 July 2022, 44 pp., doi:10.1145/3571785.3574127 | The article's own first page: "ITiCSE-WGR '22, July 8–13, 2022, Dublin, Ireland © 2022 Association for Computing Machinery"; CrossRef records the venue as "Proceedings of the 2022 Working Group Reports", published 2022-12-27 |
| C2 | "substantial evidence that Parsons problems … outperform writing-from-scratch" | The review documents the claims made in the literature and says no large-scale replication has verified them; the course may claim only that the problem type is defined, widely used and much studied | E1: "no large scale attempts at replication to verify these claims" (§1, p. 2), and RQ3 asks what the quality of the evidence is rather than asserting it (§3.1, p. 8) |
| C3 | Joentausta & Hellas, SIGCSE 2018 given as the primary source for "faded Parsons are a reasonable next step" | That paper studies subgoal-labelled worked examples with 9–10-year-olds in LightBot; the faded-Parsons row now cites Weinman, Fox & Hearst (CHI 2021) | E4's abstract names no faded Parsons problem; the faded variant's source is doi:10.1145/3411764.3445228, "Improving Instruction of Programming Patterns with Faded Parsons Problems" |

C3 is the kind of error the status column cannot catch: the citation resolved, the source
existed, and it was about the wrong intervention. What caught it was writing down what the paper
says.

## A standing rule

The status column is not decoration. A claim resting on a **named** source is a guess, and the
course says so; a claim resting on a **skimmed** source is one paper-deep. Only an entry marked
**read** would survive a reviewer asking "show me the sentence" — and it survives by having the
sentence in [Evidence](#evidence), with the pages it came from.

As of 2026-11-07, three of the five load-bearing sources are `read` (**E1**, **E2**, **E3**) and
two are `skimmed` for a stated reason (**E4** paywall, **E5** paywall). Nothing is `read` on the
strength of a title.
