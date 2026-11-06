# Parsons problem — section 15: Clients and servers

These are the lines of a working program from the lesson, **out of order**. Put them
back: the result should compile and behave like the example. Write your answer as a
single fenced block, then run `cargo test` — the crate has the real version.

```text
 1.         addr.ip()
 2.     write!(
 3. }
 4.     let mut raw = String::new();
 5.         "GET {path} HTTP/1.0\r\nHost: {}\r\n\r\n",
 6.     Ok(raw
 7.     )?;
 8.         .split_whitespace()
 9.         .and_then(|s| s.parse().ok())
10.     stream.flush()?;
11.         stream,
12.         .nth(1)
13.     let mut stream = TcpStream::connect(addr)?;
14.     stream.read_to_string(&mut raw)?;
15. pub fn get_status(addr: std::net::SocketAddr, path: &str) -> std::io::Result<u16> {
16.         .unwrap_or(0))
```

<details>
<summary>Subgoal to reach for</summary>

Name the steps before you move a line: what has to exist before what. That naming is
the skill this exercise trains, not the typing.

</details>

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
}```

</details>

<sub>Generated from `src/section_15_*.rs` by `cargo run --bin parsons` — rust_lessons 0.8.0</sub>
