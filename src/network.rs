use std::net::TcpStream;
use std::io::{Read, Write};
use std::time::Duration;

pub struct HttpClient {
    timeout: Duration,
}

impl HttpClient {
    pub fn new(timeout_secs: u64) -> Self {
        Self { timeout: Duration::from_secs(timeout_secs) }
    }

    pub fn get(&self, url: &str) -> String {
        let host = url.split("//").nth(1).unwrap().split('/').next().unwrap();
        let mut stream = TcpStream::connect(format!("{}:80", host)).unwrap();
        stream.set_read_timeout(Some(self.timeout)).unwrap();
        let request = format!("GET / HTTP/1.1\r\nHost: {}\r\n\r\n", host);
        stream.write_all(request.as_bytes()).unwrap();
        let mut response = String::new();
        stream.read_to_string(&mut response).unwrap();
        response
    }

    pub fn download_to_path(&self, url: &str, path: &str) {
        let content = self.get(url);
        std::fs::write(path, content).unwrap();
    }
}
