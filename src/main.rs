#![allow(clippy::unused_io_amount)]
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use std::{fs, thread};

use rust_web_server::ThreadPoll;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPoll::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool::execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, file_name) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 Page not found", "404.html")
    };

    // storing contents of html file in a variable
    let contents = fs::read_to_string(format!("html/{}", file_name)).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
