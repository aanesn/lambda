use crate::{
    auth::SESSION,
    db::{ConnectionPool, DatabaseConnection},
    error::AppError,
    user::User,
};
use anyhow::Context;
use axum::{
    Router,
    extract::Request,
    http::{HeaderValue, Method},
    middleware::{self, Next},
    response::Response,
    routing::get,
};
use axum_extra::extract::CookieJar;
use oauth2::{AuthUrl, ClientId, ClientSecret, EndpointNotSet, EndpointSet, RedirectUrl, TokenUrl};
use redis::AsyncCommands;
use tower_http::cors::CorsLayer;

mod auth;
mod db;
mod error;
mod user;

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
    reqwest: reqwest::Client,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let prod =
        std::env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_owned()) == "production";
    let client_url =
        std::env::var("CLIENT_URL").unwrap_or_else(|_| "http://localhost:5173".to_owned());
    let api_url = std::env::var("API_URL").unwrap_or_else(|_| "http://localhost:8080".to_owned());

    let cors = CorsLayer::new()
        .allow_methods([Method::GET])
        .allow_origin(client_url.parse::<HeaderValue>()?)
        .allow_credentials(true);

    let db = bb8::Pool::builder()
        .build(redis::Client::open(std::env::var("REDIS_URL")?)?)
        .await?;

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

    let reqwest = reqwest::Client::builder()
        .user_agent(concat!(
            env!("CARGO_PKG_NAME"),
            "/",
            env!("CARGO_PKG_VERSION"),
        ))
        .build()?;

    let ctx = Ctx {
        prod,
        client_url,
        db,
        google,
        github,
        reqwest,
    };

    let protected = Router::new()
        .route("/user", get(user::get))
        .route_layer(middleware::from_fn_with_state(ctx.clone(), auth_middleware));

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .nest("/auth", auth::mount())
        .merge(protected)
        .layer(cors)
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

async fn auth_middleware(
    jar: CookieJar,
    DatabaseConnection(mut conn): DatabaseConnection,
    mut req: Request,
    next: Next,
) -> anyhow::Result<Response, AppError> {
    let session_id = jar
        .get(SESSION)
        .map(|c| c.value())
        .ok_or(AppError::Unauthorized)?;
    let user_id: String = conn
        .get(format!("session:{session_id}"))
        .await
        .map_err(|_| AppError::Unauthorized)?;
    let user: String = conn
        .get(format!("user:{user_id}"))
        .await
        .context("failed to get user")?;
    req.extensions_mut()
        .insert(serde_json::from_str::<User>(&user)?);
    Ok(next.run(req).await)
}
