use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to serialize: {0}")]
    Json(#[from] serde_json::Error),
    #[error("no valid session found")]
    Session,
    #[error("failed to retrieve data")]
    Store,
    #[error("that game does not exist")]
    DoesNotExist,
    #[error("that game is already full")]
    Full,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let status = match self {
            Error::Json(_) => StatusCode::BAD_REQUEST,
            Error::Session => StatusCode::UNAUTHORIZED,
            Error::Store => StatusCode::INTERNAL_SERVER_ERROR,
            Error::DoesNotExist => StatusCode::NOT_FOUND,
            Error::Full => StatusCode::FORBIDDEN,
        };

        let body = Json(json!({
            "error": self.to_string(),
        }));

        (status, body).into_response()
    }
}
