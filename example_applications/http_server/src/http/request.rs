use super::Method;

pub struct Request {
    method: Method,
    path: Option<String>,
    query: String,
    headers: String,
}
