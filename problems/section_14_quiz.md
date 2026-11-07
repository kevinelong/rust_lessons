# Retrieval check — section 14: Networks, IP, DNS and URLs

Answer from memory before looking. Each question comes from a sentence in the lesson, with one term removed.

1. This section parses and validates, because parsing is where beginners meet the difference between what they meant and what they wrote: a missing port, a **____** that is empty, a hostname that is really an address.

2. A URL is four fields here rather than one string: the scheme picks the default port, so **____** alone tells a client to dial 443 and an omitted port is not missing information.

3. The **____** helper answers what will be asked and as which record type, because one name can resolve to an IPv4 answer or to an IPv6 one and the caller has to know which arrived.

4. Private ranges such as **____** route only inside a LAN, so a request to one from the public internet times out rather than failing loudly.

5. **____** means "the scheme's default", which is decided by the caller.

<details>
<summary>Answers</summary>

1. `/path`
2. `https`
3. `dns_query`
4. `192.168`
5. `None`

</details>

<sub>Generated from `src/section_14_*.rs` by `cargo run --bin quiz` — rust_lessons 0.17.0</sub>
