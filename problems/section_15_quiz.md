# Retrieval check — section 15: Clients and servers

Answer from memory before looking. Each question comes from a sentence in the lesson, with one term removed.

1. This section is a working HTTP/1.0 server and client in about a hundred lines of **____**, no dependencies.

2. Accept exactly `count` connections on **____**, answering each, then return the statuses sent.

3. A minimal client: send **____** to a bound address, return the status code.

<details>
<summary>Answers</summary>

1. `std`
2. `listener`
3. `GET path`

</details>

<sub>Generated from `src/section_15_*.rs` by `cargo run --bin quiz` — rust_lessons 0.12.0</sub>
