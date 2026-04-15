pub struct AppError {
    pub message: String,
    pub code: i32,
}

impl AppError {
    pub fn new(message: String, code: i32) -> Self {
        Self { message, code }
    }

    pub fn not_found() -> AppError {
        AppError::new("not found".to_string(), 404)
    }

    pub fn bad_request(reason: String) -> AppError {
        AppError::new(format!("bad request: {}", reason), 400)
    }
}

pub fn wrap(msg: String) -> AppError {
    AppError {
        message: msg.clone(),
        code: 500,
    }
}
