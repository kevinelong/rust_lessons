//! Introduction to programming, taught in Rust.
//!
//! The curriculum is the one from `Introduction_to_computer_programming_with_Python`
//! (the tutoring deck in `python_lessons`), lesson for lesson, rewritten for the
//! language this repository teaches. Every section is one module here, one
//! runnable example under `examples/`, and one set of tests under `tests/`.
//!
//! Read it in order, or jump with `cargo run -- <section number>`.
//!
//! # How the sections map to the deck
//!
//! | Rust module | Lesson | Deck slide |
//! |---|---|---|
//! | [`section_01_comments`] | code is written for people, comments, identifiers | 2–4 |
//! | [`section_02_variables`] | variables, `let`, `mut`, shadowing, inference | 3, 20 |
//! | [`section_03_operators`] | operators and expressions, order of operations | 5–6 |
//! | [`section_04_strings`] | numbers, text, conversion and formatting | 11–16 |
//! | [`section_05_conditionals`] | booleans, comparison, `if`/`else` | 19, 45–47 |
//! | [`section_06_enums`] | custom types from a fixed set of values | 23 |
//! | [`section_07_vectors`] | sequences: create, index, push, iterate | 26–30 |
//! | [`section_08_maps`] | lookup by key | 33–34 |
//! | [`section_09_iterators`] | ranges, laziness, map/filter/collect | 35 |
//! | [`section_10_loops`] | `for`, `while`, `loop`, `break`, `continue` | 38–44 |
//! | [`section_11_functions`] | functions, arguments, returns, D.R.Y. | 49–58 |
//! | [`section_12_capstone`] | guess the number, the first useful program | 60–62 |
//! | [`section_13_structs`] | structs, methods, traits — objects by another name | 63–67 |
//! | [`section_14_networking`] | IP, DNS, URLs | 68–71 |
//! | [`section_15_client_server`] | clients, servers, request and response | 73–76 |
//! | [`section_16_databases`] | tables, DDL, CRUD | 79–84 |
//!
//! Four more sections cover what the deck never had to say, because no Python course
//! needs to: [`section_17_ownership`], [`section_18_errors`],
//! [`section_19_input_output`], [`section_20_testing`] and
//! [`section_21_compiler_errors`] — see [`TRACK_B`].

pub mod section_01_comments;
pub mod section_02_variables;
pub mod section_03_operators;
pub mod section_04_strings;
pub mod section_05_conditionals;
pub mod section_06_enums;
pub mod section_07_vectors;
pub mod section_08_maps;
pub mod section_09_iterators;
pub mod section_10_loops;
pub mod section_11_functions;
pub mod section_12_capstone;
pub mod section_13_structs;
pub mod section_14_networking;
pub mod section_15_client_server;
pub mod section_16_databases;

// Track B: what Rust requires that the deck never had to say.
pub mod section_17_ownership;
pub mod section_18_errors;
pub mod section_19_input_output;
pub mod section_20_testing;
pub mod section_21_compiler_errors;

pub mod lesson;
pub mod version;

pub use version::VERSION;

/// The section titles, in teaching order, used by the binary's course map.
pub const COURSE_MAP: &[(&str, &str)] = &[
    ("01", "Comments and identifiers"),
    ("02", "Variables and data types"),
    ("03", "Operators and expressions"),
    ("04", "Strings, numbers and conversion"),
    ("05", "Booleans and conditionals"),
    ("06", "Enums: making your own type"),
    ("07", "Vectors: collections of values"),
    ("08", "Maps: lookup by key"),
    ("09", "Ranges and iterators"),
    ("10", "Loops"),
    ("11", "Functions"),
    ("12", "Capstone: guess the number"),
    ("13", "Structs, methods and traits"),
    ("14", "Networks, IP, DNS and URLs"),
    ("15", "Clients and servers"),
    ("16", "Tables and SQL"),
];

/// Track B — the lessons the deck has no slide for, but a Rust course cannot ship
/// without. These come after the deck's sixteen so the mapping stays honest.
pub const TRACK_B: &[(&str, &str)] = &[
    ("17", "Ownership and moves"),
    ("18", "Errors: panic, Result and ?"),
    ("19", "The keyboard and the disk"),
    ("20", "Testing, and the red-green loop"),
    ("21", "Reading compiler errors"),
];

/// How many sections the course has. One constant, so adding a section cannot leave a
/// generator or a test behind.
pub const SECTION_COUNT: u8 = 21;
