use std::fs;
use std::io::{BufReader, BufRead, Write};
use std::net::{TcpListener, TcpStream};

use multithread_web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(move || {
            handle_connection(stream);
        });
        println!("Connection Established!");
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf_reader = BufReader::new(&mut stream);
    let mut request_line = String::new();
    buf_reader.read_line(&mut request_line).unwrap();
    let request_line = request_line.trim_end().to_string();

    drop(buf_reader);

    let (status_line, filename) =
        if request_line == "GET / HTTP/1.1" {
            ("HTTP/1.1 200 OK", "hello.xhtml")
        } else {
            ("HTTP/1.1 404 NOT FOUND", "404.xhtml")
        };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}   