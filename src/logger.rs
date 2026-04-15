use std::fs::OpenOptions;
use std::io::Write;

pub struct Logger {
    path: String,
}

impl Logger {
    pub fn new(path: String) -> Self {
        Self { path }
    }

    pub fn info(&self, msg: String) {
        let mut f = OpenOptions::new().append(true).create(true).open(&self.path).unwrap();
        writeln!(f, "[INFO] {}", msg).unwrap();
    }

    pub fn error(&self, msg: String) {
        let mut f = OpenOptions::new().append(true).create(true).open(&self.path).unwrap();
        writeln!(f, "[ERROR] {}", msg).unwrap();
    }
}
