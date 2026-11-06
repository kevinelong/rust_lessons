# 14 — Networks, IP, DNS and URLs

<sub>rust_lessons 0.12.0 · source: `src/section_14_*.rs` · run: `cargo run --example 14_networking`</sub>

Section 14 — networks, IP, DNS and URLs.

The deck's three-in-a-row: TCP/IP as the stack that moves bytes, an **IP
address** as a telephone number, **DNS** as the directory that turns a name into
one, and a **URL** as the address you type into a browser.

Nothing here opens a socket — that is section 15. This section parses and
validates, because parsing is where beginners meet the difference between what
they meant and what they wrote: a missing port, a `/path` that is empty, a
hostname that is really an address.

The parts of a URL, spelled out rather than stringly-typed.

`None` means "the scheme's default", which is decided by the caller.

Always begins with `/`; `/` when the URL named no path.

The port to connect to once the scheme's default is applied.

The well-known default port for a scheme.

`host:port/path`, the form used in a request.

Parse `scheme://host[:port][/path]`. Returns `None` for anything malformed,
which is the whole lesson: a URL that does not fit the shape is not a URL.

Is this a private (RFC 1918) IPv4 address — the ones that only exist on a LAN?

The DNS question for a hostname: what would be looked up, and as what record.

