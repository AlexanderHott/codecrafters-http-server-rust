// Uncomment this block to pass the first stage
// use std::net::TcpListener;

use std::{net::TcpListener, io::{Write, Read}};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");
    let listener = TcpListener::bind("0.0.0.0:4221").unwrap();
    listener.incoming().for_each(|stream| {
        match stream {
            Ok(mut stream) => {
                let mut buf = [0u8; 1024];
                let read = stream.read(&mut buf).unwrap();
                eprintln!("Read {read} bytes {:?}", String::from_utf8(buf[..read].to_vec()));
                let written = stream.write(&"HTTP/1.1 200 OK\r\n\r\n".as_bytes()).unwrap();
                stream.flush().unwrap();
                eprintln!("Wrote {written} bytes");
            },
            Err(e) => eprintln!("error {e}"),
        }
    })
}
