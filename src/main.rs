mod http;
mod queue;

use std::{
    net::TcpListener,
    env
};
use http::handle_client;
use std::thread;

fn main() {

    let args: Vec<String> = env::args().collect();
    let host: &str = &args[1];
    let listener = TcpListener::bind(host).unwrap();

    if args.len() > 1 {
        for stream in listener.incoming() {
            thread::spawn(move || {
                let stream = stream.unwrap();
                handle_client(stream);
            });
        }
    }
    else {
        println!("Missing host argument");
    }
}




