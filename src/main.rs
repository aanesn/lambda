use crate::{auth::SESSION, database::DatabaseConnection, error::AppError, user::User};
use anyhow::Context;
use axum::{
    Router,
    extract::Request,
    middleware::{self, Next},
    response::Response,
    routing::get,
};
use axum_extra::extract::CookieJar;
use oauth2::{AuthUrl, ClientId, ClientSecret, EndpointNotSet, EndpointSet, RedirectUrl, TokenUrl};
use redis::AsyncCommands;

mod auth;
mod database;
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
    github: BasicClient,
    google: BasicClient,
    prod: bool,
    reqwest: reqwest::Client,
    db: database::ConnectionPool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let api_url = std::env::var("API_URL").unwrap_or_else(|_| "http://127.0.0.1:8080".to_owned());
    let prod =
        std::env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_owned()) == "production";

    let github = oauth_client(
        std::env::var("GITHUB_CLIENT_ID").context("missing github client id")?,
        std::env::var("GITHUB_CLIENT_SECRET").context("missing github client secret")?,
        "https://github.com/login/oauth/authorize".to_owned(),
        "https://github.com/login/oauth/access_token".to_owned(),
        format!("{api_url}/auth/github/callback"),
    )?;

    let google = oauth_client(
        std::env::var("GOOGLE_CLIENT_ID").context("missing google client id")?,
        std::env::var("GOOGLE_CLIENT_SECRET").context("missing google client secret")?,
        "https://accounts.google.com/o/oauth2/v2/auth".to_owned(),
        "https://oauth2.googleapis.com/token".to_owned(),
        format!("{api_url}/auth/google/callback"),
    )?;

    let reqwest = reqwest::Client::builder()
        .user_agent(env!("CARGO_PKG_NAME"))
        .build()
        .context("failed to build reqwest client")?;

    let db = bb8::Pool::builder()
        .build(
            redis::Client::open(std::env::var("REDIS_URL").context("missing redis url")?)
                .context("failed to open redis client")?,
        )
        .await
        .context("failed to build redis pool")?;

    let ctx = Ctx {
        github,
        google,
        prod,
        reqwest,
        db,
    };

    let protected = Router::new()
        .route("/user", get(user::get))
        .route_layer(middleware::from_fn_with_state(ctx.clone(), auth_middleware));

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .nest("/auth", auth::mount())
        .merge(protected)
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
        .set_auth_uri(AuthUrl::new(auth_url).context("failed to create auth url")?)
        .set_token_uri(TokenUrl::new(token_url).context("failed to create token url")?)
        .set_redirect_uri(RedirectUrl::new(redirect_url).context("failed to create redirect url")?))
}

async fn auth_middleware(
    jar: CookieJar,
    DatabaseConnection(mut conn): DatabaseConnection,
    mut req: Request,
    next: Next,
) -> anyhow::Result<Response, AppError> {
    let session_id = jar.get(SESSION).context("missing session")?.value();
    let user_id: String = conn
        .get::<_, Option<String>>(format!("session:{session_id}"))
        .await
        .context("failed to get user id from session")?
        .context("invalid session id")?;
    let user_str: String = conn
        .get::<_, Option<String>>(format!("user:{user_id}"))
        .await
        .context("failed to get user")?
        .context("invalid user id")?;
    let user: User = serde_json::from_str(&user_str).context("failed to deserialize user")?;
    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}
