use std::net::TcpListener;
use std::io::Read;
use std::convert::TryFrom;
use crate::http::{Request, Response, StatusCode, ParseError};

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    address: String
}

impl Server{
    pub fn new(address:String, port: String) -> Self {
        Self { address: format!("{}:{}", address, port) }
    }

    pub fn run(&self, mut handler: impl Handler) {
        let listener = TcpListener::bind(&self.address).unwrap();
        println!("Listening on {}", self.address);

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buf = [0; 1024];
                    match stream.read(&mut buf) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buf));
                            
                            let response = match Request::try_from(&buf[..]) {
                                Ok(req) => {
                                    handler.handle_request(&req)
                                },
                                Err(e) => {
                                    handler.handle_bad_request(&e)
                                }
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("failed to send response: {}", e);
                            }
                            
                        },
                        Err(e) => println!("error reading from connection: {}", e),
                    }
                },
                Err(e) => println!("error accepting connection: {}", e),
            }
        }
    }
        
}
