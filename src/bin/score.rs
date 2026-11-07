//! `score` - a recorded session's numbers: correctness, time to answer, and retrieval
//! versus recognition, per section, with an overall figure.
//!
//! It scores data that already exists; it does not ask anyone anything, open a window or
//! touch a socket. A session that does not pass [`rust_lessons::session::Session::validate`]
//! is refused by name, because a score computed from a broken session would look like a
//! result.
//!
//! Usage:
//!
//! ```text
//! cargo run --bin score                                    # the checked-in fixture
//! cargo run --bin score -- fixtures/session-2026-11-07-synthetic.json
//! cargo run --bin score -- path/to/session.json            # any session the schema accepts
//! ```

use std::process::ExitCode;

use rust_lessons::scoring::score;
use rust_lessons::session::Session;

/// The session reading is written against.
const DEFAULT_SESSION: &str = "fixtures/session-2026-11-07-synthetic.json";

fn main() -> ExitCode {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| DEFAULT_SESSION.to_string());
    match Session::load(&path) {
        Ok(session) => {
            print!("{}", score(&session).report());
            ExitCode::SUCCESS
        }
        Err(error) => {
            // `Session::load` already names the path it could not read.
            eprintln!("{error}");
            ExitCode::from(2)
        }
    }
}
