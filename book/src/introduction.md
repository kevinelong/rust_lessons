# Introduction

This is the 0.16.0 edition of a course that teaches beginning programming in Rust,
lesson for lesson from a Python deck of 85 slides. The same lessons also exist as slides,
Jupyter notebooks and narrated video — all generated from the text you are reading, so they
cannot drift from it.

## How to read it

Each chapter is one section. Read it, run its example, then do its three practice artefacts:
a Parsons problem, a faded variant, and a retrieval check.

```text
cargo run --example 07_vectors      # the lesson, running
cat problems/section_07_parsons.md  # reassemble the lines
cat problems/section_07_quiz.md     # answer from memory first
```

## What is verified, and what is not

The code in the chapters is compiled by `cargo test` (103 tests). Chapters also quote code as
`text` on purpose — it is there to be read, and it is already compiled elsewhere. The one
`rust` block below exists so that `mdbook test` compiles something rather than passing while
checking nothing.

```rust
fn greeting(name: &str) -> String {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        String::from("Hello, stranger.")
    } else {
        format!("Hello, {trimmed}.")
    }
}

fn main() {
    assert_eq!(greeting("Ada"), "Hello, Ada.");
    assert_eq!(greeting("   "), "Hello, stranger.");
}
```

The *teaching* is not verified: there is no measurement of a learner, and this course does not
claim one. `docs/STATE.md` carries the honest list.
