use serde::Serialize;

#[derive(Serialize)]
pub struct User {
    pub id: String,
    pub email: String,
}
