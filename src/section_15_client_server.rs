//! Section 15 — clients and servers.
//!
//! The deck's client/server slide: the client is the many consumers, the server is
//! the one program that answers them, and the request/response pair is the whole
//! contract between them.
//!
//! This section is a small HTTP/1.0 server and client in about a hundred lines of `std`, no
//! dependencies. It reads the **request line and headers** and answers them; it does not read
//! a request body, so a `POST` with a payload is answered on the strength of its path alone,
//! and an unknown method on a known path returns 404 rather than the 405 it deserves. Both are
//! noted where they happen — a server that is honest about its subset is more useful to read
//! than one that is described as complete. That is deliberate: sockets, a request line, headers and
//! a body are small enough to read in full, and once you have seen them a framework
//! is a convenience rather than a mystery.

use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};

/// One parsed request: the three parts of the request line, plus the headers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub version: String,
    pub headers: Vec<(String, String)>,
}

impl Request {
    /// Parse the first line (and the headers that follow it) of an HTTP request.
    pub fn parse(raw: &str) -> Option<Request> {
        let mut lines = raw.lines();
        let first = lines.next()?;
        let mut parts = first.split_whitespace();
        let method = parts.next()?.to_string();
        let path = parts.next()?.to_string();
        let version = parts.next().unwrap_or("HTTP/1.0").to_string();
        if !version.starts_with("HTTP/") {
            return None;
        }
        let mut headers = Vec::new();
        for line in lines {
            if line.trim().is_empty() {
                break;
            }
            if let Some((k, v)) = line.split_once(':') {
                headers.push((k.trim().to_lowercase(), v.trim().to_string()));
            }
        }
        Some(Request {
            method,
            path,
            version,
            headers,
        })
    }

    /// The value of a header, compared case-insensitively.
    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(k, _)| k == &name.to_lowercase())
            .map(|(_, v)| v.as_str())
    }
}

/// A response the server knows how to send.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Response {
    pub status: u16,
    pub reason: &'static str,
    pub body: String,
    pub content_type: &'static str,
}

impl Response {
    /// A 200 with the given body.
    pub fn ok(body: impl Into<String>) -> Self {
        Response {
            status: 200,
            reason: "OK",
            body: body.into(),
            content_type: "text/plain; charset=utf-8",
        }
    }

    /// A 200 carrying JSON, the deck's "app consumes simple data formats".
    pub fn json(body: impl Into<String>) -> Self {
        Response {
            status: 200,
            reason: "OK",
            body: body.into(),
            content_type: "application/json",
        }
    }

    /// A 404, the only error this server can produce.
    pub fn not_found() -> Self {
        Response {
            status: 404,
            reason: "Not Found",
            body: String::from("no such path"),
            content_type: "text/plain; charset=utf-8",
        }
    }

    /// The bytes on the wire: status line, headers, blank line, body.
    pub fn to_http(&self) -> String {
        format!(
            "HTTP/1.0 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            self.status,
            self.reason,
            self.content_type,
            self.body.as_bytes().len(),
            self.body
        )
    }
}

/// The application: which path maps to which response.
///
/// This stands in for the router a framework would give you — and it is the exact
/// place the deck says the interesting design lives.
pub fn route(request: &Request) -> Response {
    let known = ["/", "/time", "/courses", "/health"];
    if request.method != "GET" && known.contains(&request.path.as_str()) {
        // A known path with the wrong method is 405, not 404: the difference is the whole
        // reason status codes exist, and the AI review that flagged the 404 caught it.
        return Response {
            status: 405,
            reason: "Method Not Allowed",
            body: String::from("this server only speaks GET"),
            content_type: "text/plain; charset=utf-8",
        };
    }
    match (request.method.as_str(), request.path.as_str()) {
        ("GET", "/") => Response::ok("rust_lessons server: try /time or /courses"),
        ("GET", "/time") => Response::json(r#"{"epoch":"whenever you asked"}"#),
        ("GET", "/courses") => Response::json(format!(
            r#"{{"sections":{},"version":"{}"}}"#,
            crate::COURSE_MAP.len(),
            crate::VERSION
        )),
        ("GET", "/health") => Response::ok("ok"),
        _ => Response::not_found(),
    }
}

/// Read one request from a stream and write one response. Returns the status sent.
pub fn handle_one(stream: &mut TcpStream) -> std::io::Result<u16> {
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut head = String::new();
    loop {
        let mut line = String::new();
        if reader.read_line(&mut line)? == 0 {
            break;
        }
        let blank = line.trim().is_empty();
        head.push_str(&line);
        if blank {
            break;
        }
    }
    let response = match Request::parse(&head) {
        Some(request) => route(&request),
        None => Response::not_found(),
    };
    stream.write_all(response.to_http().as_bytes())?;
    stream.flush()?;
    Ok(response.status)
}

/// Accept exactly `count` connections on `listener`, answering each, then return
/// the statuses sent. The test uses this with a port of 0 so it never collides.
pub fn serve_n(listener: &TcpListener, count: usize) -> std::io::Result<Vec<u16>> {
    let mut statuses = Vec::new();
    for _ in 0..count {
        let (mut stream, _addr) = listener.accept()?;
        statuses.push(handle_one(&mut stream)?);
    }
    Ok(statuses)
}

/// A minimal client: send `GET path` to a bound address, return the status code.
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::SocketAddr;

    #[test]
    fn requests_parse_into_a_method_path_and_headers() {
        let raw = "GET /courses HTTP/1.1\r\nHost: example.com\r\nAccept: */*\r\n\r\n";
        let r = Request::parse(raw).expect("valid request");
        assert_eq!(r.method, "GET");
        assert_eq!(r.path, "/courses");
        assert_eq!(r.header("host"), Some("example.com"));
        assert_eq!(r.header("HOST"), Some("example.com"));
        assert_eq!(r.header("missing"), None);
        assert!(Request::parse("this is not http").is_none());
    }

    #[test]
    fn responses_are_well_formed() {
        let http = Response::ok("hello").to_http();
        assert!(http.starts_with("HTTP/1.0 200 OK\r\n"));
        assert!(http.contains("Content-Length: 5\r\n"));
        assert!(http.ends_with("\r\n\r\nhello"));
        assert_eq!(Response::not_found().status, 404);
    }

    #[test]
    fn routing_maps_paths_to_responses() {
        let req = |m: &str, p: &str| Request {
            method: m.to_string(),
            path: p.to_string(),
            version: String::from("HTTP/1.0"),
            headers: vec![],
        };
        assert_eq!(route(&req("GET", "/health")).body, "ok");
        assert_eq!(route(&req("GET", "/courses")).content_type, "application/json");
        assert_eq!(route(&req("GET", "/nope")).status, 404);
        assert_eq!(
            route(&req("POST", "/health")).status,
            405,
            "known path, wrong method is 405 (the 404 was a real defect)"
        );
        assert_eq!(route(&req("GET", "/nope")).status, 404);
    }

    #[test]
    fn a_real_socket_carries_a_real_request() {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind loopback");
        let addr: SocketAddr = listener.local_addr().unwrap();
        let server = std::thread::spawn(move || serve_n(&listener, 2).unwrap());
        // The client may connect before the server thread is scheduled; the
        // listener is already bound, so the kernel queues the connection.
        let first = get_status(addr, "/health").unwrap();
        let second = get_status(addr, "/nope").unwrap();
        assert_eq!((first, second), (200, 404));
        assert_eq!(server.join().unwrap(), vec![200, 404]);
    }
}
