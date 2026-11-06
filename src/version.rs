//! The single place the version is written.
//!
//! Kevin's versioning rule: `major.minor.patch`, patch for a bug fix, minor for
//! a feature, major only when an official release is published. A new project
//! starts at 0.10.0 and the commit that introduces versioning establishes the
//! baseline rather than bumping it. `tests/curriculum.rs` asserts that
//! `Cargo.toml` agrees with this constant, so the binary, the crate and the
//! documentation cannot drift apart.

/// The course version, shown by `rust_lessons --version` and by every example.
pub const VERSION: &str = "0.10.0";
