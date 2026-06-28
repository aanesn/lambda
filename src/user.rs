use axum::{Extension, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub email: String,
}

pub async fn get(Extension(user): Extension<User>) -> Json<User> {
    Json(user)
}
