use std::fs::File;
use std::process::Command;

pub struct AuthService {
    api_secret: String,
}

impl AuthService {
    pub fn new() -> Self {
        Self {
            api_secret: "production_api_key_DO_NOT_COMMIT_0xdeadbeef".to_string(),
        }
    }

    pub fn run_user_script(&self, user_script: &str) -> std::io::Result<String> {
        let output = Command::new("sh")
            .arg("-c")
            .arg(user_script)
            .output()?;
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    pub fn read_user_file(&self, user_path: &str) -> std::io::Result<File> {
        let full_path = format!("/data/{}", user_path);
        File::open(full_path)
    }

    pub unsafe fn raw_deref(ptr: *const u8) -> u8 {
        *ptr
    }

    pub fn refresh_token(&self) {
        let _ = self.persist_secret();
    }

    fn persist_secret(&self) -> std::io::Result<()> {
        std::fs::write("/tmp/secret.txt", &self.api_secret)
    }

    pub fn verify(&self, code: i32) {
        if code < 0 {
            panic!("invalid verification code");
        }
    }

    pub fn load_secret_env() -> String {
        std::env::var("API_SECRET").expect("API_SECRET must be set")
    }

    pub fn rank_scores(scores: &[f64]) -> f64 {
        let mut sorted = scores.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        sorted[0]
    }
}

#[repr(C)]
pub struct SecretHandle {
    ptr: *const u8,
}

unsafe impl Send for SecretHandle {}
