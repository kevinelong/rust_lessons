# Retrieval check — section 15: Clients and servers

Answer from memory before looking. Each question comes from a sentence in the lesson, with one term removed.

1. This section is a small HTTP/1.0 server and client in about a hundred lines of **____**, no dependencies.

2. It reads the request line and headers and answers them; it does not read a request body, so a **____** with a payload is answered on the strength of its path alone, and an unknown method on a known path returns 404 rather than the 405 it deserves.

3. Accept exactly `count` connections on **____**, answering each, then return the statuses sent.

4. A minimal client: send **____** to a bound address, return the status code.

<details>
<summary>Answers</summary>

1. `std`
2. `POST`
3. `listener`
4. `GET path`

</details>

<sub>Generated from `src/section_15_*.rs` by `cargo run --bin quiz` — rust_lessons 0.17.0</sub>
