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
        let path = match after_scheme.find('/') {
            Some(pos) => &after_scheme[pos..],
            None => "/",
        };
        let mut stream = TcpStream::connect(format!("{}:80", host)).map_err(|e| e.to_string())?;
        stream.set_read_timeout(Some(self.timeout)).map_err(|e| e.to_string())?;
        let request = format!("GET {} HTTP/1.1\r\nHost: {}\r\n\r\n", path, host);
        stream.write_all(request.as_bytes()).map_err(|e| e.to_string())?;
        let mut response = String::new();
        stream.read_to_string(&mut response).map_err(|e| e.to_string())?;
        Ok(response)
    }

    pub fn download_to_path(&self, url: &str, path: &str, allowed_dir: &std::path::Path) -> Result<(), String> {
        let target = std::path::Path::new(path);

        // Resolve the allowed directory to its canonical form.
        let canonical_base = allowed_dir
            .canonicalize()
            .map_err(|e| format!("invalid base directory: {}", e))?;

        // Ensure parent directory of target exists so we can canonicalize up to it,
        // then join the file name. This prevents `..` components from escaping.
        let parent = target.parent().unwrap_or_else(|| std::path::Path::new("."));
        let canonical_parent = parent
            .canonicalize()
            .map_err(|e| format!("cannot resolve parent directory of path: {}", e))?;
        let file_name = target
            .file_name()
            .ok_or_else(|| "path does not contain a valid file name".to_string())?;
        let canonical_target = canonical_parent.join(file_name);

        if !canonical_target.starts_with(&canonical_base) {
            return Err(format!(
                "path traversal denied: {:?} is outside allowed directory {:?}",
                canonical_target, canonical_base
            ));
        }

        let content = self.get(url)?;
        std::fs::write(&canonical_target, content).map_err(|e| e.to_string())?;
        Ok(())
    }
}
// re-trigger
// test context removal 1775971513
