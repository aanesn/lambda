use crate::database::ConnectionPool;
use axum::{
    Router,
    routing::{get, post},
};

mod database;
mod error;
mod webhook;

#[derive(Clone)]
struct Ctx {
    pool: ConnectionPool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let pool = bb8::Pool::builder()
        .build(redis::Client::open(std::env::var("REDIS_URL")?)?)
        .await?;

    let ctx = Ctx { pool };

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/webhook", post(webhook::post))
        .with_state(ctx);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;
    println!("listening on http://{}", listener.local_addr()?);
    Ok(axum::serve(listener, app).await?)
}
