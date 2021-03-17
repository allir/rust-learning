use std::io::{Write, Result as IoResult};
use super::StatusCode;

pub struct Response {
    protocol: String,
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        let protocol = "HTTP/1.1".to_string();
        Response { protocol, status_code, body}
    }

    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };

        write!(stream, "{} {} {}\r\n\r\n{}",
            self.protocol,
            self.status_code,
            self.status_code.reason_phrase(),
            body)
    }
}
