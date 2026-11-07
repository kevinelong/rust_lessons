# 15 — Clients and servers

<sub>rust_lessons 0.16.0 · source: `src/section_15_*.rs` · run: `cargo run --example 15_client_server`</sub>

Section 15 — clients and servers.

The deck's client/server slide: the client is the many consumers, the server is
the one program that answers them, and the request/response pair is the whole
contract between them.

This section is a small HTTP/1.0 server and client in about a hundred lines of `std`, no
dependencies. It reads the **request line and headers** and answers them; it does not read
a request body, so a `POST` with a payload is answered on the strength of its path alone,
and an unknown method on a known path returns 404 rather than the 405 it deserves. Both are
noted where they happen — a server that is honest about its subset is more useful to read
than one that is described as complete. That is deliberate: sockets, a request line, headers and
a body are small enough to read in full, and once you have seen them a framework
is a convenience rather than a mystery.

One parsed request: the three parts of the request line, plus the headers.

Parse the first line (and the headers that follow it) of an HTTP request.

The value of a header, compared case-insensitively.

A response the server knows how to send.

A 200 with the given body.

A 200 carrying JSON, the deck's "app consumes simple data formats".

A 404, the only error this server can produce.

The bytes on the wire: status line, headers, blank line, body.

The application: which path maps to which response.

This stands in for the router a framework would give you — and it is the exact
place the deck says the interesting design lives.

Read one request from a stream and write one response. Returns the status sent.

Accept exactly `count` connections on `listener`, answering each, then return
the statuses sent. The test uses this with a port of 0 so it never collides.

A minimal client: send `GET path` to a bound address, return the status code.

