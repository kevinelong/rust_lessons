# Faded Parsons problem — section 15: Clients and servers

The **structure is given and the decisions are missing**. Fill the blanks so the function compiles and behaves like the lesson's version. Each blank is a decision point: something has to exist before that line can run, and naming that is the exercise.

```rust
pub fn get_status(addr: std::net::SocketAddr, path: &str) -> std::io::Result<u16> {
    // ______  (blank 1)
    write!(
        stream,
        "GET {path} HTTP/1.0\r\nHost: {}\r\n\r\n",
        addr.ip()
    )?;
    stream.flush()?;
    // ______  (blank 2)
    stream.read_to_string(&mut raw)?;
    Ok(raw
        .split_whitespace()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(0))
}
```

## The blanks

1. `let mut stream = TcpStream::connect(addr)?;` — what has to exist before this line runs?
2. `let mut raw = String::new();` — what has to exist before this line runs?

<details>
<summary>Canonical order (do not peek first)</summary>

```rust
pub fn get_status(addr: std::net::SocketAddr, path: &str) -> std::io::Result<u16> {
    let mut stream = TcpStream::connect(addr)?;
    write!(
        stream,
        "GET {path} HTTP/1.0\r\nHost: {}\r\n\r\n",
        addr.ip()
    )?;
    stream.flush()?;
    let mut raw = String::new();
    stream.read_to_string(&mut raw)?;
    Ok(raw
        .split_whitespace()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(0))
}
```

</details>

<sub>Generated from `src/section_15_*.rs` by `cargo run --bin parsons` — rust_lessons 0.9.0
</sub>
