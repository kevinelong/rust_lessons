//! Section 15, running: a real server and a real client on one machine.
//!
//! `cargo run --example 15_client_server`
//!
//! The server runs on a thread against `127.0.0.1:0` (a port the kernel picks), so
//! this example can be run repeatedly and never collides with anything.

use std::net::{SocketAddr, TcpListener};
use std::thread;

use rust_lessons::section_15_client_server::{get_status, serve_n, Request, Response};

fn main() {
    println!("rust_lessons {} — section 15", rust_lessons::VERSION);

    // The parsing side, with no network involved.
    let raw = "GET /courses HTTP/1.1\r\nHost: localhost\r\n\r\n";
    let request = Request::parse(raw).expect("well-formed request");
    println!("  parsed: {} {} ({})", request.method, request.path, request.version);
    println!("  content-type the router chose: {}", Response::json("{}").content_type);

    // The real thing.
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind loopback");
    let addr: SocketAddr = listener.local_addr().unwrap();
    println!("  serving on http://{addr}/");

    let server = thread::spawn(move || serve_n(&listener, 4).unwrap());
    for path in ["/", "/health", "/courses", "/nope"] {
        let status = get_status(addr, path).expect("request");
        println!("  GET {path:<10} -> {status}");
    }
    let statuses = server.join().expect("server thread");
    println!("  server answered {statuses:?}");

    println!();
    println!("  For the long-running version: cargo run -- serve 8765");
}
