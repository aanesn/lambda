use crate::{Ctx, error::AppError};
use anyhow::Context;
use axum::{
    Router,
    extract::{Query, State},
    response::{IntoResponse, Redirect},
    routing::get,
};
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};
use oauth2::{AuthorizationCode, CsrfToken, Scope, TokenResponse};
use serde::Deserialize;

const CSRF_TOKEN: &str = "csrf_token";

pub fn mount() -> Router<Ctx> {
    Router::new()
        .route("/github", get(github))
        .route("/google", get(google))
        .route("/github/callback", get(github_callback))
        .route("/google/callback", get(google_callback))
}

async fn github(State(ctx): State<Ctx>, jar: CookieJar) -> impl IntoResponse {
    let (auth_url, csrf_token) = ctx
        .github
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("user:email".to_owned()))
        .url();
    (
        set_cookie(
            CSRF_TOKEN,
            csrf_token.into_secret(),
            ctx.prod,
            time::Duration::minutes(10),
            jar,
        ),
        Redirect::to(auth_url.as_ref()),
    )
}

async fn google(State(ctx): State<Ctx>, jar: CookieJar) -> impl IntoResponse {
    let (auth_url, csrf_token) = ctx
        .google
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("email".to_owned()))
        .url();
    (
        set_cookie(
            CSRF_TOKEN,
            csrf_token.into_secret(),
            ctx.prod,
            time::Duration::minutes(10),
            jar,
        ),
        Redirect::to(auth_url.as_ref()),
    )
}

fn set_cookie(
    name: &'static str,
    value: String,
    prod: bool,
    max_age: time::Duration,
    jar: CookieJar,
) -> CookieJar {
    let cookie = Cookie::build((name, value))
        .path("/")
        .secure(prod)
        .http_only(true)
        .same_site(SameSite::Lax)
        .max_age(max_age);
    jar.add(cookie)
}

#[derive(Deserialize)]
struct CallbackParams {
    code: String,
    state: String,
}

#[derive(Deserialize)]
struct GithubUser {
    id: i64,
}

#[derive(Deserialize)]
struct GithubEmail {
    email: String,
    verified: bool,
    primary: bool,
}

async fn github_callback(
    jar: CookieJar,
    Query(params): Query<CallbackParams>,
    State(ctx): State<Ctx>,
) -> anyhow::Result<impl IntoResponse, AppError> {
    check_csrf_token(&jar, &params.state)?;

    let token = ctx
        .github
        .exchange_code(AuthorizationCode::new(params.code))
        .request_async(&ctx.reqwest)
        .await
        .context("failed to get github token")?;

    let user = ctx
        .reqwest
        .get("https://api.github.com/user")
        .bearer_auth(token.access_token().secret())
        .send()
        .await
        .context("failed to get github user")?
        .json::<GithubUser>()
        .await
        .context("failed to parse github user")?;

    let email = ctx
        .reqwest
        .get("https://api.github.com/user/emails")
        .bearer_auth(token.access_token().secret())
        .send()
        .await
        .context("failed to get github emails")?
        .json::<Vec<GithubEmail>>()
        .await
        .context("failed to parse github emails")?
        .into_iter()
        .find(|e| e.verified && e.primary)
        .context("missing verified and primary github email")?
        .email;

    Ok("test".to_string())
}

#[derive(Deserialize)]
struct GoogleUser {
    sub: String,
    email: String,
    email_verified: bool,
}

async fn google_callback(
    jar: CookieJar,
    Query(params): Query<CallbackParams>,
    State(ctx): State<Ctx>,
) -> anyhow::Result<impl IntoResponse, AppError> {
    check_csrf_token(&jar, &params.state)?;

    let token = ctx
        .google
        .exchange_code(AuthorizationCode::new(params.code))
        .request_async(&ctx.reqwest)
        .await
        .context("failed to get google token")?;

    let user = ctx
        .reqwest
        .get("https://openidconnect.googleapis.com/v1/userinfo")
        .bearer_auth(token.access_token().secret())
        .send()
        .await
        .context("failed to get google user")?
        .json::<GoogleUser>()
        .await
        .context("failed to parse google user")?;

    if !user.email_verified {
        return Err(anyhow::anyhow!("missing verified google email").into());
    }

    Ok("test".to_string())
}

fn check_csrf_token(jar: &CookieJar, state: &str) -> anyhow::Result<()> {
    let stored_state = jar
        .get(CSRF_TOKEN)
        .context("failed to get csrf token")?
        .value();
    if stored_state != state {
        anyhow::bail!("csrf token mismatch")
    }
    Ok(())
}
