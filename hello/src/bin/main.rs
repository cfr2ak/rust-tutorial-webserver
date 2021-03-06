extern crate hello;
use hello::ThreadPool;

use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs::File;
use std::thread;
use std::time::Duration;

fn main() {
    // bind (vocav in tcp domain) == new (for the others)
    // bind(new) returns Result<T, E>
    // so, it needed to be unwraped (in here the call of method unwrap())
    //     unwrap() stops program if E returned
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let pool = ThreadPool::new(4);

    // TcpListener::incoming() returns iterator
    //     that gives streams of type TcpStream
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    } // stream drops here by TcpStream

}

fn handle_connection(mut stream: TcpStream) {
    let mut buf = [0; 512];
    stream.read(&mut buf).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buf.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
    } else if buf.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
    } else {
        //println!("not found: {}", buf);
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let res = format!("{}{}", status_line, contents);
    stream.write(res.as_bytes()).unwrap();
    stream.flush().unwrap();
}
