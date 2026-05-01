// Bug 09: shell command injection — user input interpolated into a shell string.
use std::process::Command;

pub fn echo(message: &str) -> std::io::Result<std::process::Output> {
    Command::new("sh")
        .arg("-c")
        .arg(format!("echo {}", message))
        .output()
}
