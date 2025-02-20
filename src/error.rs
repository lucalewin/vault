use axum::response::IntoResponse;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Generic error: {0}")]
    Generic(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        axum::response::Json(self.to_string()).into_response()
    }
}

pub type Result<T> = std::result::Result<T, Error>;
