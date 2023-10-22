#![allow(dead_code, unused_variables)]
pub mod http;
use std::{
    // io::{Read, Write},
    // net::{TcpListener, TcpStream},
    str::FromStr,
};
use tokio::{
    net::{TcpStream, TcpListener},
    io::{AsyncWriteExt, AsyncReadExt},
};

use crate::http::{Headers, StatusLine};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");
    let listener = TcpListener::bind("0.0.0.0:4221").await?;
    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_request(stream));
        // match stream {
        //     Ok(mut stream) => {
        //         handle_request(&mut stream).unwrap();
        //     }
        //     Err(e) => eprintln!("error {e}"),
        // }
    }
    Ok(())
}

async fn handle_request(mut stream: TcpStream) -> anyhow::Result<()> {
    let mut buf = [0u8; 1024];
    let read = stream.read(&mut buf).await?;

    let req_str = String::from_utf8(buf[..read].to_vec())?;
    let (status_line, rest) = req_str.split_once("\r\n").unwrap();

    let req = rest.split("\r\n\r\n").collect::<Vec<_>>();
    let headers = req[0];
    // let body = req[2];
    eprintln!("Read {read} bytes {status_line:?} {headers:?}");

    let status_line = StatusLine::from_str(status_line)?;
    let headers = Headers::from_str(headers)?;
    eprintln!("Read {read} bytes {status_line:?} {headers:?}");

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
        "user-agent" => {
            let s = headers.0.get("User-Agent").unwrap();
            response.push_str(format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}\r\n\r\n", s.len(), s).as_str());
        }
        p => {
            eprintln!("Not route for {}", p);
            response.push_str("HTTP/1.1 404 Not Found\r\n\r\n");
        }
    };

    eprintln!("writing response\n{response}");
    let written = stream.write(response.as_bytes()).await?;
    stream.flush().await?;
    eprintln!("Wrote {written} bytes");
    Ok(())
}
