use axum::response::IntoResponse;

pub async fn post(body: String) -> impl IntoResponse {
    println!("{}", body);
    "test".to_string()
}
