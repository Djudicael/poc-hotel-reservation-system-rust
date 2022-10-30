use axum::{http::StatusCode, BoxError, Json};
use serde_json::json;

pub mod observability;

pub const HTTP_TIMEOUT: u64 = 30;

pub async fn handle_timeout_error(err: BoxError) -> (StatusCode, Json<serde_json::Value>) {
    if err.is::<tower::timeout::error::Elapsed>() {
        (
            StatusCode::REQUEST_TIMEOUT,
            Json(json!({
                "error":
                    format!(
                        "request took longer than the configured {} second timeout",
                        HTTP_TIMEOUT
                    )
            })),
        )
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": format!("unhandled internal error: {}", err)
            })),
        )
    }
}
