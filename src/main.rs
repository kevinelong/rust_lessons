//! `rust_lessons` — the course as a program.
//!
//! `rust_lessons` alone prints the map of the sixteen sections.
//! `rust_lessons 7` prints one section's lesson text.
//! `rust_lessons --version` prints the version, which lives in one place.
//! `rust_lessons serve [port]` runs the section 15 server so you can talk to it.

use std::net::TcpListener;
use std::process::ExitCode;

use rust_lessons::{COURSE_MAP, VERSION};

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        None => {
            print_map();
            ExitCode::SUCCESS
        }
        Some("--version") | Some("-V") | Some("version") => {
            println!("rust_lessons {VERSION}");
            ExitCode::SUCCESS
        }
        Some("--help") | Some("-h") | Some("help") => {
            usage();
            ExitCode::SUCCESS
        }
        Some("serve") => {
            let port: u16 = args.get(1).and_then(|p| p.parse().ok()).unwrap_or(8765);
            serve(port)
        }
        Some(other) => match other.parse::<u8>() {
            Ok(n) if (1..=16).contains(&n) => {
                print_section(n as usize);
                ExitCode::SUCCESS
            }
            _ => {
                eprintln!("no such section: {other}");
                usage();
                ExitCode::from(2)
            }
        },
    }
}

fn usage() {
    println!("usage: rust_lessons [1-16 | --version | serve [port]]");
    println!("       with no argument, print the course map");
}

fn print_map() {
    println!("rust_lessons {VERSION} — introduction to programming, in Rust");
    println!();
    for (n, title) in COURSE_MAP {
        println!("  {n}  {title}");
    }
    println!();
    println!("  cargo run -- 7              the lesson text for section 7");
    println!("  cargo run --example 07_vectors   the code, running");
    println!("  cargo test                  every section's tests");
    println!("  cargo run -- serve          the section 15 server on :8765");
}

/// One paragraph per section — the deck's teaching sentence, in Rust terms.
fn lesson(n: u8) -> (&'static str, &'static str) {
    match n {
        1 => ("Comments and identifiers", "Names are for readers. snake_case, no leading digit, no keywords. `///` documents."),
        2 => ("Variables and data types", "`let` binds and is immutable until you write `mut`. Every value has a type, inferred when omitted."),
        3 => ("Operators and expressions", "`+ - * / %` with arithmetic precedence. Parenthesise when the order matters."),
        4 => ("Strings, numbers and conversion", "`&str` borrows, `String` owns. `parse` returns a Result, so bad text is an error you must handle."),
        5 => ("Booleans and conditionals", "Conditions are `bool`; there is no truthiness. `if` is an expression and both arms share a type."),
        6 => ("Enums", "A type whose value is one of a named set. `match` must be exhaustive, and variants can carry data."),
        7 => ("Vectors", "`Vec<T>` is the growable list: same type throughout, `get` returns Option, indexing panics."),
        8 => ("Maps", "`HashMap<K, V>` looks up by key. `entry().or_insert()` is how you count; `get` never panics."),
        9 => ("Ranges and iterators", "`1..=10` is a recipe, not a list. `map` and `filter` are lazy until `collect`, `sum` or `for`."),
        10 => ("Loops", "`for` visits, `while` repeats on a condition, `loop` runs until `break` — which can return a value."),
        11 => ("Functions", "Parameters and return types are declared, the last expression is the value. Borrowing keeps calls cheap."),
        12 => ("Capstone", "Guess the number: a secret, a verdict, a narrowing range, and a win counted in attempts."),
        13 => ("Structs, methods and traits", "A struct holds properties, an impl holds methods, a trait is the shared contract."),
        14 => ("Networks, IP, DNS and URLs", "An address to reach a machine, a directory to find it by name, a URL to name a thing on it."),
        15 => ("Clients and servers", "One program listens, many connect. A request line, headers, a body; a status, headers, a body."),
        _ => ("Tables and SQL", "Tables, DDL to shape them, and CRUD — insert, select, update, delete — each with its SQL."),
    }
}

fn print_section(n: usize) {
    let (nstr, title) = COURSE_MAP[n - 1];
    let (heading, body) = lesson(n as u8);
    println!("{nstr} — {heading} ({title})");
    println!();
    println!("{body}");
    println!();
    println!("  lesson and tests: src/section_{nstr}_*.rs");
    println!("  run it:           cargo run --example {nstr}_*");
}

/// Serve the section 15 router on `port`, forever, one connection at a time.
fn serve(port: u16) -> ExitCode {
    let addr = format!("127.0.0.1:{port}");
    let listener = match TcpListener::bind(&addr) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("cannot bind {addr}: {e}");
            return ExitCode::from(1);
        }
    };
    println!("rust_lessons {VERSION} serving http://{addr}/ (ctrl-c to stop)");
    println!("  GET /  /health  /time  /courses");
    for incoming in listener.incoming() {
        match incoming {
            Ok(mut stream) => {
                let peer = stream
                    .peer_addr()
                    .map(|a| a.to_string())
                    .unwrap_or_else(|_| String::from("?"));
                match rust_lessons::section_15_client_server::handle_one(&mut stream) {
                    Ok(status) => println!("  {peer} -> {status}"),
                    Err(e) => println!("  {peer} -> error {e}"),
                }
            }
            Err(e) => println!("  accept failed: {e}"),
        }
    }
    ExitCode::SUCCESS
}
