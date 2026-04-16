use std::fs::File;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct AuthService {
    api_secret: String,
}

impl AuthService {
    pub fn new() -> std::io::Result<Self> {
        let api_secret = std::env::var("API_SECRET")
            .map_err(|e| Error::new(ErrorKind::NotFound, format!("API_SECRET missing: {}", e)))?;
        Ok(Self { api_secret })
    }

    pub fn run_user_script(&self, script_path: &str) -> std::io::Result<String> {
        let path = Path::new(script_path);
        if path.components().any(|c| matches!(c, std::path::Component::ParentDir | std::path::Component::RootDir)) {
            return Err(Error::new(ErrorKind::InvalidInput, "script path must be relative"));
        }
        let base = PathBuf::from("/opt/scripts");
        let full_path = base.join(path);
        let output = Command::new("bash").arg(full_path).output()?;
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    pub fn read_user_file(&self, user_path: &str) -> std::io::Result<File> {
        let path = Path::new(user_path);
        if path.components().any(|c| matches!(c, std::path::Component::ParentDir | std::path::Component::RootDir)) {
            return Err(Error::new(ErrorKind::InvalidInput, "path traversal not allowed"));
        }
        let base = PathBuf::from("/data");
        let full_path = base.join(path);
        File::open(full_path)
    }

    /// Dereferences a raw pointer.
    ///
    /// # Safety
    /// The caller must ensure that `ptr` is non-null, properly aligned,
    /// points to a valid initialized `u8`, and is not aliased mutably
    /// for the duration of the call.
    pub unsafe fn raw_deref(ptr: *const u8) -> u8 {
        *ptr
    }

    pub fn refresh_token(&self) -> std::io::Result<()> {
        self.persist_secret()
    }

    fn persist_secret(&self) -> std::io::Result<()> {
        let dir = std::env::var("SECRET_DIR").unwrap_or_else(|_| "/var/lib/app".to_string());
        let path = PathBuf::from(dir).join("secret.txt");
        std::fs::write(path, &self.api_secret)
    }

    pub fn verify(&self, code: i32) -> Result<(), String> {
        if code < 0 {
            return Err(format!("invalid verification code: {}", code));
        }
        Ok(())
    }

    pub fn load_secret_env() -> Result<String, std::env::VarError> {
        std::env::var("API_SECRET")
    }

    pub fn rank_scores(scores: &[f64]) -> Option<f64> {
        let mut sorted = scores.to_vec();
        sorted.sort_by(|a, b| a.total_cmp(b));
        sorted.first().copied()
    }
}

pub struct SecretHandle {
    secret: String,
}

impl SecretHandle {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }

    pub fn reveal(&self) -> &str {
        &self.secret
    }
}
