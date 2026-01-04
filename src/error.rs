use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub enum ApiError {
    Internal(anyhow::Error),
    Unauthorized,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::Internal(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            ApiError::Unauthorized => (StatusCode::UNAUTHORIZED, "unauthorized".to_owned()),
        }
        .into_response()
    }
}

impl<E> From<E> for ApiError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self::Internal(err.into())
    }
}
