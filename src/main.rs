#![allow(dead_code, unused_variables)]
pub mod http;
use std::{
    env,
    path::PathBuf,
    // io::{Read, Write},
    // net::{TcpListener, TcpStream},
    str::FromStr,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

use crate::http::{Headers, StatusLine};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    let directory = if args.len() == 3 && args[1] == "--directory" {
        args[2].to_string()
    } else {
        "".to_string()
    };

    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");
    let listener = TcpListener::bind("0.0.0.0:4221").await?;
    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_request(stream, directory.to_owned()));
        // match stream {
        //     Ok(mut stream) => {
        //         handle_request(&mut stream).unwrap();
        //     }
        //     Err(e) => eprintln!("error {e}"),
        // }
    }
    Ok(())
}

async fn handle_request(mut stream: TcpStream, dir: String) -> anyhow::Result<()> {
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
        "files" => {
            let file_path = PathBuf::from(dir + "/" + &path_parts[2..].join("/"));
            eprintln!("Path: {:?}", file_path);
            if let Ok(file_contents) = tokio::fs::read_to_string(file_path).await {
                // eprintln!("file contents: {:?}", file_contents);
                response.push_str(
                    format!("HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n{}\r\n\r\n", 
                    file_contents.len(), file_contents).as_str());
            } else {
                response.push_str("HTTP/1.1 404 Not Found\r\n\r\n");
            }
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
