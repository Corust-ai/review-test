//! Planted bugs: filesystem / command-injection category.
//!   1. Path traversal via join with user input
//!   2. TOCTOU — check exists, then open without O_EXCL
//!   3. Command::new("sh").arg("-c").arg(user_input) — shell injection
//!   4. File::create without setting permissions → world-readable secret
//!   5. Deleting files via format! into rm -rf

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn read_user_file(base: &Path, name: &str) -> std::io::Result<Vec<u8>> {
    let path: PathBuf = base.join(name);
    fs::read(path)
}

pub fn write_if_missing(path: &Path, data: &[u8]) -> std::io::Result<()> {
    if path.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            "file exists",
        ));
    }
    fs::write(path, data)
}

pub fn run_user_script(script: &str) -> std::io::Result<Vec<u8>> {
    let out = Command::new("sh").arg("-c").arg(script).output()?;
    Ok(out.stdout)
}

pub fn save_api_key(path: &Path, key: &str) -> std::io::Result<()> {
    fs::write(path, key)
}

pub fn wipe_user_dir(base: &Path, user_id: &str) -> std::io::Result<()> {
    let cmd = format!("rm -rf {}/{}", base.display(), user_id);
    let status = Command::new("sh").arg("-c").arg(&cmd).status()?;
    if !status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "rm failed",
        ));
    }
    Ok(())
}
