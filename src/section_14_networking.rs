//! Section 14 — networks, IP, DNS and URLs.
//!
//! The deck's three-in-a-row: TCP/IP as the stack that moves bytes, an **IP
//! address** as a telephone number, **DNS** as the directory that turns a name into
//! one, and a **URL** as the address you type into a browser.
//!
//! Nothing here opens a socket — that is section 15. This section parses and
//! validates, because parsing is where beginners meet the difference between what
//! they meant and what they wrote: a missing port, a `/path` that is empty, a
//! hostname that is really an address.

/// The parts of a URL, spelled out rather than stringly-typed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Url {
    pub scheme: String,
    pub host: String,
    /// `None` means "the scheme's default", which is decided by the caller.
    pub port: Option<u16>,
    /// Always begins with `/`; `/` when the URL named no path.
    pub path: String,
}

impl Url {
    /// The port to connect to once the scheme's default is applied.
    pub fn effective_port(&self) -> u16 {
        match self.port {
            Some(p) => p,
            None => Self::default_port(&self.scheme),
        }
    }

    /// The well-known default port for a scheme.
    pub fn default_port(scheme: &str) -> u16 {
        match scheme {
            "http" | "ws" => 80,
            "https" | "wss" => 443,
            "ssh" => 22,
            _ => 0,
        }
    }

    /// `host:port/path`, the form used in a request.
    pub fn authority(&self) -> String {
        match self.port {
            Some(p) => format!("{}:{}", self.host, p),
            None => self.host.clone(),
        }
    }
}

/// Parse `scheme://host[:port][/path]`. Returns `None` for anything malformed,
/// which is the whole lesson: a URL that does not fit the shape is not a URL.
pub fn parse_url(text: &str) -> Option<Url> {
    let (scheme, rest) = text.split_once("://")?;
    if scheme.is_empty() || !scheme.chars().all(|c| c.is_ascii_alphabetic()) {
        return None;
    }
    let (authority, path) = match rest.find('/') {
        Some(i) => (&rest[..i], &rest[i..]),
        None => (rest, "/"),
    };
    let (host, port) = match authority.rsplit_once(':') {
        Some((h, p)) if !p.is_empty() && p.chars().all(|c| c.is_ascii_digit()) => {
            (h, Some(p.parse::<u16>().ok()?))
        }
        Some((h, _)) => (h, None),
        None => (authority, None),
    };
    if host.is_empty() || host.contains(char::is_whitespace) {
        return None;
    }
    Some(Url {
        scheme: scheme.to_lowercase(),
        host: host.to_lowercase(),
        port,
        path: path.to_string(),
    })
}

/// Is this a private (RFC 1918) IPv4 address — the ones that only exist on a LAN?
pub fn is_private_ipv4(addr: &str) -> bool {
    let octets: Vec<u8> = addr
        .split('.')
        .filter_map(|p| p.parse::<u8>().ok())
        .collect();
    if octets.len() != 4 {
        return false;
    }
    match (octets[0], octets[1]) {
        (10, _) => true,
        (172, second) if (16..=31).contains(&second) => true,
        (192, 168) => true,
        (127, _) => true, // loopback, not private but equally not routable
        _ => false,
    }
}

/// The DNS question for a hostname: what would be looked up, and as what record.
pub fn dns_query(name: &str) -> Option<(String, &'static str)> {
    let name = name.trim().to_lowercase();
    if name.is_empty() {
        return None;
    }
    if is_private_ipv4(&name) || name.parse::<std::net::Ipv6Addr>().is_ok() {
        return Some((name, "PTR (reverse lookup)"));
    }
    if !name.contains('.') {
        return Some((name, "search suffix applies"));
    }
    Some((name, "A / AAAA"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn urls_parse_into_parts() {
        let u = parse_url("https://Example.COM/a/b?x=1").unwrap();
        assert_eq!(u.scheme, "https");
        assert_eq!(u.host, "example.com");
        assert_eq!(u.port, None);
        assert_eq!(u.path, "/a/b?x=1");
        assert_eq!(u.effective_port(), 443);
        assert_eq!(u.authority(), "example.com");
    }

    #[test]
    fn explicit_ports_win_and_paths_default_to_root() {
        let u = parse_url("http://localhost:8080").unwrap();
        assert_eq!(u.port, Some(8080));
        assert_eq!(u.path, "/");
        assert_eq!(u.effective_port(), 8080);
        assert_eq!(u.authority(), "localhost:8080");
    }

    #[test]
    fn malformed_urls_are_rejected() {
        assert!(parse_url("no-scheme-here").is_none());
        assert!(parse_url("://host/path").is_none());
        assert!(parse_url("http:///path").is_none());
        assert!(parse_url("http://host:notaport/path").is_some(), "port falls back to None");
    }

    #[test]
    fn private_addresses_are_recognised() {
        assert!(is_private_ipv4("10.0.0.5"));
        assert!(is_private_ipv4("172.16.4.4"));
        assert!(is_private_ipv4("192.168.1.1"));
        assert!(!is_private_ipv4("8.8.8.8"));
        assert!(!is_private_ipv4("not.an.address"));
        assert_eq!(dns_query("10.0.0.5").unwrap().1, "PTR (reverse lookup)");
        assert_eq!(dns_query("example.com").unwrap().1, "A / AAAA");
    }
}
