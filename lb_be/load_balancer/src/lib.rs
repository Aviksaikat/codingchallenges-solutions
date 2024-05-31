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
    // let buf_reader = BufReader::new(&mut stream);
    // let http_request: Vec<_> = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();

    // // println!("Request: {:#?}", http_request);
    // //* get the peer address
    // let peer_addr = stream
    //     .peer_addr()
    //     .unwrap_or_else(|_| "unknown".parse().unwrap())
    //     .to_string();

    // //* strip the port from the peer address
    // if peer_addr.len() > 1 {
    //     let host_line = &peer_addr;
    //     let host_parts: Vec<&str> = host_line.split(':').collect();
    //     if host_parts.len() > 1 {
    //         let host = host_parts[0].trim();
    //         println!("Received request from {}", host);
    //     }
    // }

    // for elem in http_request.iter() {
    //     println!("{}", elem);
    // }
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

    let mut backend_response = String::new();
    let mut backend_reader = BufReader::new(backend_stream);
    backend_reader.read_to_string(&mut backend_response).unwrap();
    backend_response
}