use chapter20::ThreadPool;
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    let term = Arc::new(Mutex::new(false));

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let term1 = Arc::clone(&term);
        pool.execute(move || {
            if handle_connection(stream) {
                let mut termination = term1.lock().unwrap();
                *termination = true;
            }
        });

        if *term.lock().unwrap() {
            break;
        }
    }
}

fn handle_connection(mut stream: TcpStream) -> bool {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let terminate = b"GET /terminate HTTP/1.1\r\n";
    let (status_line, filename, term) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html", false)
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html", false)
    } else if buffer.starts_with(terminate) {
        ("HTTP/1.1 503 Service Unavailable\r\n\r\n", "503.html", true)
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html", false)
    };
    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    term
}
