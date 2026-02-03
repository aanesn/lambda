use crate::db::ConnectionPool;
use axum::{Router, routing::get};
use oauth2::{AuthUrl, ClientId, ClientSecret, EndpointNotSet, EndpointSet, RedirectUrl, TokenUrl};

mod auth;
mod db;
mod error;

type BasicClient = oauth2::basic::BasicClient<
    EndpointSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointSet,
>;

#[derive(Clone)]
struct Ctx {
    prod: bool,
    client_url: String,
    db: ConnectionPool,
    google: BasicClient,
    github: BasicClient,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let prod =
        std::env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_owned()) == "production";
    let client_url =
        std::env::var("CLIENT_URL").unwrap_or_else(|_| "http://localhost:5173".to_owned());

    let db = bb8::Pool::builder()
        .build(redis::Client::open(std::env::var("REDIS_URL")?)?)
        .await?;

    let google = oauth_client(
        std::env::var("GOOGLE_CLIENT_ID")?,
        std::env::var("GOOGLE_CLIENT_SECRET")?,
        "https://accounts.google.com/o/oauth2/v2/auth".to_owned(),
        "https://oauth2.googleapis.com/token".to_owned(),
        format!("{client_url}/auth/google/callback"),
    )?;

    let github = oauth_client(
        std::env::var("GITHUB_CLIENT_ID")?,
        std::env::var("GITHUB_CLIENT_SECRET")?,
        "https://github.com/login/oauth/authorize".to_owned(),
        "https://github.com/login/oauth/access_token".to_owned(),
        format!("{client_url}/auth/github/callback"),
    )?;

    let ctx = Ctx {
        prod,
        client_url,
        db,
        google,
        github,
    };

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .nest("/auth", auth::mount())
        .with_state(ctx);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;
    println!("listening on http://{}", listener.local_addr()?);
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
