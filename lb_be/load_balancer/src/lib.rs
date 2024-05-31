use std::{
    io::{prelude::*, BufReader, BufRead},
    net::{TcpListener, TcpStream},
};

pub fn start_load_balaner() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

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

    let backend_response = forward_to_backend(&request);

    stream.write_all(backend_response.as_bytes()).unwrap();
}

fn forward_to_backend(request: &str) -> String {
    let mut backend_stream = TcpStream::connect("127.0.0.1:7879").unwrap();
    backend_stream.write_all(request.as_bytes()).unwrap();

    let mut backend_response = vec![];
    let mut backend_reader = BufReader::new(backend_stream);
    backend_reader.read_until(*"\r\n\r\n".as_bytes(), &mut backend_response).unwrap();
    let backend_response = String::from_utf8_lossy(&backend_response);
    backend_response.to_string()
}
