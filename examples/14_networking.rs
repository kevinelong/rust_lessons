//! Section 14, running: URLs, IP addresses and DNS questions.
//!
//! `cargo run --example 14_networking`

use rust_lessons::section_14_networking::{dns_query, is_private_ipv4, parse_url};

fn main() {
    println!("rust_lessons {} — section 14", rust_lessons::VERSION);

    for text in [
        "https://Example.COM/a/b?x=1",
        "http://localhost:8080",
        "https://example.com",
        "no-scheme-here",
        "http:///path",
    ] {
        match parse_url(text) {
            Some(u) => println!(
                "  {text:<28} scheme={} host={} port={:?} path={} -> {}",
                u.scheme,
                u.host,
                u.port,
                u.path,
                u.effective_port()
            ),
            None => println!("  {text:<28} not a URL"),
        }
    }

    println!();
    for a in ["10.0.0.5", "172.16.4.4", "192.168.1.1", "127.0.0.1", "8.8.8.8", "nope"] {
        println!("  {a:<14} private/loopback? {}", is_private_ipv4(a));
    }

    println!();
    for name in ["example.com", "localhost", "10.0.0.5", "::1"] {
        println!("  {name:<12} -> {:?}", dns_query(name));
    }
}
