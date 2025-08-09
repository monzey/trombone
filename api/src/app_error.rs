use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub struct AppError {
    code: StatusCode,
    message: String,
}

impl AppError {
    pub fn new(code: StatusCode, message: &str) -> Self {
        Self {
            code,
            message: message.to_string(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (self.code, self.message).into_response()
    }
}
