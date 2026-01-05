use axum::{Extension, Json};
use serde::{Deserialize, Serialize};

#[typeshare::typeshare]
#[derive(Deserialize, Serialize, Clone)]
pub struct User {
    pub id: String,
    pub email: String,
}

pub async fn get(Extension(user): Extension<User>) -> Json<User> {
    Json(user)
}
