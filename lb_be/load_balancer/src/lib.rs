use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

pub fn connection() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    // println!("Request: {:#?}", http_request);
    //* get the peer address
    let peer_addr = stream
        .peer_addr()
        .unwrap_or_else(|_| "unknown".parse().unwrap())
        .to_string();

    //* strip the port from the peer address
    if peer_addr.len() > 1 {
        let host_line = &peer_addr;
        let host_parts: Vec<&str> = host_line.split(':').collect();
        if host_parts.len() > 1 {
            let host = host_parts[0].trim();
            println!("Received request from {}", host);
        }
    }

    for elem in http_request.iter() {
        println!("{}", elem);
    }
}
