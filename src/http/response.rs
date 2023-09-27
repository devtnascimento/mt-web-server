use std::fs;
use std::result::Result;
use std::error::Error;
use std::net::TcpStream;
use std::io::Write;
use std::thread;
use std::sync::{Arc, Mutex};
use crate::http::error_handler::HttpError;
use crate::queue::*;


pub struct Response {
    stream: TcpStream,
    request: Vec<String>,
    action: String,
    content_type: &'static str,
    header: String,
    body: String,
    pub log: Vec<Box<dyn Error>>,
    http_errors: Vec<Box<dyn Error>>,
    pub request_queue: Arc::<RequestQueue<Vec<String>>>
}

impl Response {

    pub fn new(request: Vec<String>, stream: TcpStream) -> Self{
        Self {
            stream,
            request,
            content_type: "",
            action: String::from(""),
            header: String::from(""),
            body: String::from(""),
            log: Vec::new(),
            http_errors: Vec::new(),
            request_queue: Arc::new(RequestQueue::new())
        }
    }

    fn set_action(&mut self) {
        let first: String = self.request[0].clone();
        let strings: Vec<&str> = first.split_whitespace().collect();
        self.action = strings[1].to_string();
    }

    fn set_body(&mut self) {
        match self.action.as_str() {
            "/" | "/html" => {
                self.body = fs::read_to_string("src/index.html").unwrap();
                self.content_type = "Content-Type: text/html\r\n";
            }
            "/js" => {
                match self.check_src() {
                    Ok(path) => {
                        self.body = fs::read_to_string(path).unwrap();
                    }
                    Err(err) => {
                        if err.is::<HttpError>() {
                            self.http_errors.push(err);
                        }
                        else {
                            self.log.push(err);
                        }
                    }
                };
                self.content_type = "Content-Type: text/js\r\n";
            }
            "/image" => {
                match self.check_src() {
                    Ok(path) => {
                        self.body = fs::read_to_string(path).unwrap();
                        self.read_image();
                        self.content_type = "Content-Type: image/js\r\n";
                    }
                    Err(err) => {
                        if err.is::<HttpError>() {
                            self.http_errors.push(err);
                        }
                        else {
                            self.log.push(err);
                        }
                    }
                };                
            }
            _ => {self.http_errors.push(Box::new(HttpError::NotImplemented));}
        }
    }

    fn check_src(&self) -> Result<String, Box<dyn Error>> {
        for line in &self.request {
            let elements: Vec<&str> = line.split_whitespace().collect();
            let key = elements[0].trim();
            let value: &str = elements[1].trim();
            
            if key == "file-name:" {
                match fs::metadata(value) {
                    Ok(_) => {
                        return Ok(value.to_string());
                    }
                    Err(_) => {
                        return Err(Box::new(HttpError::NotFound));
                    }
                }
            }
        }
        return Err(Box::new(HttpError::BadRequest));
    }

    fn set_header(&mut self) {

        if self.log.len() + self.http_errors.len() == 0 {
            let status_line = "HTTP/1.1 200 OK\r\n";
            let content_length = format!("content-length: {}\r\n", self.body.len());

            self.header = format!("{}{}{}\r\n{}",
                status_line,
                self.content_type,
                content_length,
                self.body
            );
        }
        else if self.http_errors.len() > 0 {
            let status_line = self.http_errors[0].to_string().clone();
            self.header = format!("{}\r\n",
                status_line
            );
        }
        else if self.log.len() > 0 {
            let status_line = HttpError::InternalServer.to_string();
            self.header = format!("{}\r\n",
                status_line
            );
        }

    }

    pub fn set_priority(&mut self) -> i32 {
        self.set_action();

        if self.action == "/" {3}
        else if self.action == "/js" {2}
        else if self.action == "/js" {1}
        else {0}
    }

    pub fn handle_request(&mut self, http_request: Vec<String>) {
        println!("handle_request");
        let priority = self.set_priority();
        let request = PriorityItem::new(http_request.clone(), priority);
        self.request_queue.push_back(request.item, request.priority);
    }

    pub fn consume(&self) {
        println!("self queue{:?}", self.request_queue.queue);
        let queue_ref = Arc::clone(&self.request_queue); 
        println!("queue ref {:?}", queue_ref.queue);
        thread::spawn(move || {
            loop {
                println!("Consumindo a fila...");
                if let Some(request) = queue_ref.queue.lock().unwrap().pop() {

                } else {
                    println!("Fila vazia...");
                    thread::sleep(std::time::Duration::from_secs(1));
                }
            }
        });
    }

    pub fn send(&mut self) {
        self.set_action();
        self.set_body();
        self.set_header();
        let response = format!("{}{}", self.header, self.body);
        self.stream.write_all(response.as_bytes()).unwrap();
    }

    fn read_image(&mut self) {}

}

