use axum::{Extension, Json, response::IntoResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
    pub avatar_url: Option<String>,
}

pub async fn get(Extension(user): Extension<User>) -> impl IntoResponse {
    Json(user)
}
