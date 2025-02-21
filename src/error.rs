use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Generic error: {0}")]
    Generic(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        Json(serde_json::json!({
            "status": "error",
            "message": self.to_string()
        })).into_response()
    }
}

// pub type Result<T> = std::result::Result<T, Error>;
