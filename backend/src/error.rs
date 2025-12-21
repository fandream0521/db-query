use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    DatabaseError(String),
    ValidationError(String),
    NotFound(String),
    InternalError(String),
    ConnectionError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message, code) = match self {
            AppError::DatabaseError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                msg,
                Some("DATABASE_ERROR".to_string()),
            ),
            AppError::ValidationError(msg) => (
                StatusCode::BAD_REQUEST,
                msg,
                Some("VALIDATION_ERROR".to_string()),
            ),
            AppError::NotFound(msg) => (
                StatusCode::NOT_FOUND,
                msg,
                Some("NOT_FOUND".to_string()),
            ),
            AppError::InternalError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                msg,
                Some("INTERNAL_ERROR".to_string()),
            ),
            AppError::ConnectionError(msg) => (
                StatusCode::BAD_REQUEST,
                msg,
                Some("CONNECTION_ERROR".to_string()),
            ),
        };

        let mut body = json!({
            "error": error_message,
        });
        
        if let Some(code) = code {
            body["code"] = json!(code);
        }

        (status, Json(body)).into_response()
    }
}

impl From<rusqlite::Error> for AppError {
    fn from(err: rusqlite::Error) -> Self {
        tracing::error!("[AppError] SQLite error: {:?}", err);
        AppError::DatabaseError(err.to_string())
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        // Check if it's a connection error
        if matches!(err, sqlx::Error::PoolClosed | sqlx::Error::Io(_)) {
            tracing::error!("[AppError] SQLx connection error: {:?}", err);
            AppError::ConnectionError(err.to_string())
        } else {
            tracing::error!("[AppError] SQLx database error: {:?}", err);
            AppError::DatabaseError(err.to_string())
        }
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::InternalError(err.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::InternalError(format!("JSON parsing error: {err}"))
    }
}

