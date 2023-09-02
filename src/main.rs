use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    fs,
};
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}


// --snip--

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let status_line = "HTTP/1.1 200 OK";
    let response =|contents: String| {
        let length = contents.len();
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}")};

    if request_line == "GET / HTTP/1.1" {
        print!("{request_line}");
        let contents = fs::read_to_string("hello.html").unwrap();
        stream.write_all(response(contents).as_bytes()).unwrap();
    } else {
        let contents = fs::read_to_string("info.html").unwrap();
        stream.write_all(response(contents).as_bytes()).unwrap()
    }
}

