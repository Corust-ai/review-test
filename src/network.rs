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

    pub fn get(&self, url: &str) -> Result<String, String> {
        let after_scheme = url.split("//").nth(1).ok_or_else(|| format!("malformed URL: {}", url))?;
        let host = after_scheme.split('/').next().ok_or_else(|| format!("malformed URL: {}", url))?;
        let mut stream = TcpStream::connect(format!("{}:80", host)).map_err(|e| e.to_string())?;
        stream.set_read_timeout(Some(self.timeout)).map_err(|e| e.to_string())?;
        let request = format!("GET / HTTP/1.1\r\nHost: {}\r\n\r\n", host);
        stream.write_all(request.as_bytes()).map_err(|e| e.to_string())?;
        let mut response = String::new();
        stream.read_to_string(&mut response).map_err(|e| e.to_string())?;
        Ok(response)
    }

    pub fn download_to_path(&self, url: &str, path: &str) -> Result<(), String> {
        let content = self.get(url)?;
        std::fs::write(path, content).map_err(|e| e.to_string())?;
        Ok(())
    }
}
// re-trigger
