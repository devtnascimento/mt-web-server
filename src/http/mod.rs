pub mod response;
pub mod error_handler;


use std::{
    io::{prelude::*, BufReader},
    net::TcpStream,
};
use response::Response;
use crate::queue::*;


pub fn handle_client(stream: TcpStream) {
    println!("handle_connection call");

    let mut buffer = Vec::new();
    let mut request = String::new();

    match BufReader::new(&stream).read_until(b'\n', &mut buffer) {
        Ok(_) => {

            let http_request: Vec<String> = buffer
                .split(|&byte| byte == b'\n')
                .map(|line| String::from_utf8_lossy(line).to_string())
                .collect();

            println!("Received request: {:?}", http_request);


            let mut response = Response::new(http_request.clone(), stream);
            response.handle_request(http_request);
            println!("{:?}", response.request_queue.queue);
            response.consume();

        }
        Err(e) => {
            eprintln!("Error reading from client: {}", e);
        }
    }
}
