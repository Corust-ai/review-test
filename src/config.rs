use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub port: u16,
    pub max_connections: u32,
    pub debug: bool,
    pub allowed_origins: Vec<String>,
}

impl AppConfig {
    /// Load config from a .env-style file.
    pub fn from_file(path: &str) -> Result<Self, ConfigError> {
        let content = fs::read_to_string(path)
            .map_err(|e| ConfigError::IoError(e.to_string()))?;
        let map = parse_env_file(&content);
        Self::from_map(&map)
    }

    /// Load config from environment variables.
    pub fn from_env() -> Result<Self, ConfigError> {
        Ok(Self {
            database_url: std::env::var("DATABASE_URL")
                .map_err(|_| ConfigError::Missing("DATABASE_URL".into()))?,
            port: std::env::var("PORT")
                .unwrap_or_else(|_| "8080".into())
                .parse()
                .map_err(|_| ConfigError::Invalid("PORT".into(), "must be a number".into()))?,
            max_connections: std::env::var("MAX_CONNECTIONS")
                .unwrap_or_else(|_| "10".into())
                .parse()
                .map_err(|_| ConfigError::Invalid("MAX_CONNECTIONS".into(), "must be a number".into()))?,
            debug: std::env::var("DEBUG")
                .unwrap_or_else(|_| "false".into())
                .parse()
                .unwrap_or(false),
            allowed_origins: std::env::var("ALLOWED_ORIGINS")
                .unwrap_or_default()
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect(),
        })
    }

    fn from_map(map: &HashMap<String, String>) -> Result<Self, ConfigError> {
        let database_url = map.get("DATABASE_URL")
            .ok_or_else(|| ConfigError::Missing("DATABASE_URL".into()))?
            .clone();

        // BUG: port parsing uses unwrap — panics on invalid input
        let port: u16 = map.get("PORT")
            .unwrap_or(&"8080".to_string())
            .parse()
            .unwrap();

        let max_connections: u32 = map.get("MAX_CONNECTIONS")
            .unwrap_or(&"10".to_string())
            .parse()
            .map_err(|_| ConfigError::Invalid("MAX_CONNECTIONS".into(), "must be a number".into()))?;

        let debug = map.get("DEBUG")
            .map(|v| v == "true" || v == "1")
            .unwrap_or(false);

        let allowed_origins = map.get("ALLOWED_ORIGINS")
            .map(|v| v.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect())
            .unwrap_or_default();

        Ok(Self { database_url, port, max_connections, debug, allowed_origins })
    }
}

#[derive(Debug)]
pub enum ConfigError {
    Missing(String),
    Invalid(String, String),
    IoError(String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Missing(key) => write!(f, "missing required config: {}", key),
            Self::Invalid(key, reason) => write!(f, "invalid config '{}': {}", key, reason),
            Self::IoError(msg) => write!(f, "config I/O error: {}", msg),
        }
    }
}

impl std::error::Error for ConfigError {}

/// Parse a .env-style file into key-value pairs.
/// Supports `KEY=VALUE`, ignores comments (#) and blank lines.
fn parse_env_file(content: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        // BUG: split('=') instead of splitn(2, '=') — truncates values containing '='
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim().to_string();
            let value = value.trim().to_string();
            // Strip surrounding quotes if present
            let value = if (value.starts_with('"') && value.ends_with('"'))
                || (value.starts_with('\'') && value.ends_with('\''))
            {
                value[1..value.len() - 1].to_string()
            } else {
                value
            };
            map.insert(key, value);
        }
    }
    map
}

/// Validate a database URL format.
pub fn validate_db_url(url: &str) -> bool {
    url.starts_with("postgres://") || url.starts_with("postgresql://")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_env_basic() {
        let content = "KEY=value\nDB=postgres://localhost/test\n# comment\n";
        let map = parse_env_file(content);
        assert_eq!(map.get("KEY").unwrap(), "value");
        assert_eq!(map.get("DB").unwrap(), "postgres://localhost/test");
    }

    #[test]
    fn test_parse_env_quoted() {
        let content = r#"SECRET="hello world""#;
        let map = parse_env_file(content);
        assert_eq!(map.get("SECRET").unwrap(), "hello world");
    }

    #[test]
    fn test_validate_db_url() {
        assert!(validate_db_url("postgres://localhost/db"));
        assert!(validate_db_url("postgresql://user:pass@host/db"));
        assert!(!validate_db_url("mysql://localhost/db"));
        assert!(!validate_db_url(""));
    }
}
