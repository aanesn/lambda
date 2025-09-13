use axum::{Router, routing::get};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("db url is not set");

    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("failed to connect to db");

    let app = Router::new().route("/", get(|| async { "hello world!" }));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    println!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
