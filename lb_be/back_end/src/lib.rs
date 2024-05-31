use std::io::{BufReader, BufRead, Write};
use std::net::{TcpListener, TcpStream};

pub fn start_backend_server() {
    let listener = TcpListener::bind("127.0.0.1:7879").unwrap();
    println!("Backend server listening on port 7879");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let peer_addr = stream.peer_addr().unwrap();
    println!("Received request from: {:?}", peer_addr);

    let mut buf_reader = BufReader::new(&stream);
    let mut request = String::new();
    buf_reader.read_line(&mut request).unwrap();

    println!("Request: {}", request);

    let response = "HTTP/1.1 200 OK\r\nContent-Length: 23\r\n\r\nHello From Backend Server\r\n";
    stream.write_all(response.as_bytes()).unwrap();
    println!("Replied with a hello message");
}
