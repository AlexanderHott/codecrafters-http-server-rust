#![allow(dead_code, unused_variables)]
// Uncomment this block to pass the first stage
// use std::net::TcpListener;

use anyhow::anyhow;
use std::{
    io::{Read, Write},
    net::TcpListener,
    str::FromStr,
};

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum Method {
    Get,
}

impl FromStr for Method {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match s {
            "GET" => Ok(Method::Get),
            m => Err(anyhow!("Not implemented for {m}")),
        };
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum HttpVersion {
    Http11,
}

impl FromStr for HttpVersion {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match s {
            "HTTP/1.1" => Ok(HttpVersion::Http11),
            v => Err(anyhow!("Not implemented for {}", v)),
        };
    }
}

#[derive(Debug)]
struct StatusLine {
    method: Method,
    path: String,
    version: HttpVersion,
}

impl FromStr for StatusLine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_ascii_whitespace();
        let method = Method::from_str(words.next().unwrap())?;
        let path = words.next().unwrap();
        let version = HttpVersion::from_str(words.next().unwrap())?;
        return Ok(Self {
            method,
            path: path.to_string(),
            version,
        });
    }
}

fn main() -> anyhow::Result<()> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");
    let listener = TcpListener::bind("0.0.0.0:4221").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buf = [0u8; 1024];
                let read = stream.read(&mut buf).unwrap();
                if let Some((status_line, rest)) = String::from_utf8(buf[..read].to_vec())
                    .unwrap()
                    .split_once("\r\n")
                {
                    let status_line = StatusLine::from_str(status_line)?;
                    eprintln!("Read {read} bytes {status_line:?}");

                    let mut response = String::new();
                    eprintln!("{:?}", status_line);
                    let path_parts = status_line.path.split("/").collect::<Vec<_>>();
                    eprintln!("{:?}", path_parts);
                    match path_parts[1] {
                        "" => {
                            response.push_str("HTTP/1.1 200 OK\r\n\r\n");
                        }
                        "echo" => {
                            let s = &path_parts[2..].join("/");
                            eprintln!("s {s}");
                            response.push_str(format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}\r\n\r\n", s.len(), s).as_str());
                        }
                        p => {
                            eprintln!("Not route for {}", p);
                            response.push_str("HTTP/1.1 404 Not Found\r\n\r\n");
                        }
                    };
                    eprintln!("writing response\n{response}");
                    let written = stream.write(response.as_bytes()).unwrap();
                    stream.flush().unwrap();
                    eprintln!("Wrote {written} bytes");
                }
            }
            Err(e) => eprintln!("error {e}"),
        }
    }
    Ok(())
}
