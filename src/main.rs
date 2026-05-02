use crate::database::ConnectionPool;
use axum::{
    Router,
    routing::{get, post},
};
use oauth2::{AuthUrl, ClientId, ClientSecret, EndpointNotSet, EndpointSet, RedirectUrl, TokenUrl};

mod database;
mod error;
mod webhook;

type BasicClient = oauth2::basic::BasicClient<
    EndpointSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointSet,
>;

#[derive(Clone)]
struct Ctx {
    pool: ConnectionPool,
    google: BasicClient,
    github: BasicClient,
    prod: bool,
    client_url: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let pool = bb8::Pool::builder()
        .build(redis::Client::open(std::env::var("REDIS_URL")?)?)
        .await?;

    let prod =
        std::env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_owned()) == "production";
    let client_url =
        std::env::var("CLIENT_URL").unwrap_or_else(|_| "http://localhost:5173".to_owned());
    let api_url = std::env::var("API_URL").unwrap_or_else(|_| "http://localhost:8080".to_owned());

    let google = oauth_client(
        std::env::var("GOOGLE_CLIENT_ID")?,
        std::env::var("GOOGLE_CLIENT_SECRET")?,
        "https://accounts.google.com/o/oauth2/v2/auth".to_owned(),
        "https://oauth2.googleapis.com/token".to_owned(),
        format!("{api_url}/auth/google/callback"),
    )?;

    let github = oauth_client(
        std::env::var("GITHUB_CLIENT_ID")?,
        std::env::var("GITHUB_CLIENT_SECRET")?,
        "https://github.com/login/oauth/authorize".to_owned(),
        "https://github.com/login/oauth/access_token".to_owned(),
        format!("{api_url}/auth/github/callback"),
    )?;

    let ctx = Ctx {
        pool,
        google,
        github,
        prod,
        client_url,
    };

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/webhook", post(webhook::post))
        .with_state(ctx);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;
    println!("listening on http://localhost:8080");
    Ok(axum::serve(listener, app).await?)
}

fn oauth_client(
    client_id: String,
    client_secret: String,
    auth_url: String,
    token_url: String,
    redirect_url: String,
) -> anyhow::Result<BasicClient> {
    Ok(oauth2::basic::BasicClient::new(ClientId::new(client_id))
        .set_client_secret(ClientSecret::new(client_secret))
        .set_auth_uri(AuthUrl::new(auth_url)?)
        .set_token_uri(TokenUrl::new(token_url)?)
        .set_redirect_uri(RedirectUrl::new(redirect_url)?))
}
