use crate::{Ctx, db::DatabaseConnection, error::ApiError, user::User};
use anyhow::Context;
use axum::{
    Router,
    extract::{Query, State},
    response::{IntoResponse, Redirect},
    routing::get,
};
use axum_extra::extract::{CookieJar, cookie::Cookie, cookie::SameSite};
use oauth2::{AuthorizationCode, CsrfToken, Scope, TokenResponse};
use redis::AsyncCommands;
use serde::Deserialize;

const CSRF_TOKEN: &str = "csrf_token";
const CSRF_TOKEN_MAX_AGE: i64 = 60 * 10; // 10 min
pub const SESSION_ID: &str = "session_id";
const SESSION_ID_MAX_AGE: i64 = 24 * 60 * 60 * 7; // 7 days

pub fn mount() -> Router<Ctx> {
    Router::new()
        .route("/google", get(google))
        .route("/github", get(github))
        .route("/google/callback", get(google_callback))
        .route("/github/callback", get(github_callback))
        .route("/logout", get(logout))
}

pub async fn google(
    State(ctx): State<Ctx>,
    jar: CookieJar,
) -> anyhow::Result<impl IntoResponse, ApiError> {
    let (auth_url, csrf_token) = ctx
        .google
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("openid".to_owned()))
        .add_scope(Scope::new("email".to_owned()))
        .url();
    Ok((
        set_cookie(
            jar,
            CSRF_TOKEN,
            csrf_token.secret().to_owned(),
            CSRF_TOKEN_MAX_AGE,
            ctx.prod,
        ),
        Redirect::to(auth_url.as_ref()),
    ))
}

pub async fn github(
    State(ctx): State<Ctx>,
    jar: CookieJar,
) -> anyhow::Result<impl IntoResponse, ApiError> {
    let (auth_url, csrf_token) = ctx
        .github
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("user:email".to_owned()))
        .url();
    Ok((
        set_cookie(
            jar,
            CSRF_TOKEN,
            csrf_token.secret().to_owned(),
            CSRF_TOKEN_MAX_AGE,
            ctx.prod,
        ),
        Redirect::to(auth_url.as_ref()),
    ))
}

#[derive(Deserialize)]
pub struct AuthParams {
    code: String,
    state: String,
}

#[derive(Deserialize)]
struct GoogleUser {
    sub: String,
    email: String,
    email_verified: bool,
}

pub async fn google_callback(
    jar: CookieJar,
    Query(params): Query<AuthParams>,
    State(ctx): State<Ctx>,
    DatabaseConnection(mut conn): DatabaseConnection,
) -> anyhow::Result<impl IntoResponse, ApiError> {
    check_csrf_token(&jar, &params.state)?;
    let token = ctx
        .google
        .exchange_code(AuthorizationCode::new(params.code.clone()))
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
        return Err(ApiError::Unauthorized);
    }
    let user_id = set_user("google", user.sub, user.email, &mut conn).await?;
    let session_id = set_session(&mut conn, user_id).await?;
    Ok((
        set_cookie(jar, SESSION_ID, session_id, SESSION_ID_MAX_AGE, ctx.prod),
        Redirect::to("/user"),
    ))
}

#[derive(Deserialize)]
struct GithubUser {
    id: i64,
}

#[derive(Deserialize)]
struct GithubEmail {
    email: String,
    primary: bool,
    verified: bool,
}

pub async fn github_callback(
    jar: CookieJar,
    Query(params): Query<AuthParams>,
    State(ctx): State<Ctx>,
    DatabaseConnection(mut conn): DatabaseConnection,
) -> anyhow::Result<impl IntoResponse, ApiError> {
    check_csrf_token(&jar, &params.state)?;
    let token = ctx
        .github
        .exchange_code(AuthorizationCode::new(params.code.clone()))
        .request_async(&ctx.reqwest)
        .await
        .context("failed to get github token")?;
    let access_token = token.access_token().secret();
    let user = ctx
        .reqwest
        .get("https://api.github.com/user")
        .bearer_auth(access_token)
        .send()
        .await
        .context("failed to get github user")?
        .json::<GithubUser>()
        .await
        .context("failed to parse github user")?;
    let email = ctx
        .reqwest
        .get("https://api.github.com/user/emails")
        .bearer_auth(access_token)
        .send()
        .await
        .context("failed to get github emails")?
        .json::<Vec<GithubEmail>>()
        .await
        .context("failed to parse github emails")?
        .into_iter()
        .find(|e| e.primary && e.verified)
        .ok_or(ApiError::Unauthorized)?
        .email;
    let user_id = set_user("github", user.id.to_string(), email, &mut conn).await?;
    let session_id = set_session(&mut conn, user_id).await?;
    Ok((
        set_cookie(jar, SESSION_ID, session_id, SESSION_ID_MAX_AGE, ctx.prod),
        Redirect::to("/user"),
    ))
}

async fn logout(
    jar: CookieJar,
    DatabaseConnection(mut conn): DatabaseConnection,
) -> anyhow::Result<impl IntoResponse, ApiError> {
    if let Some(id) = jar.get(SESSION_ID).map(|c| c.value()) {
        conn.del::<_, ()>(format!("session:{id}")).await?;
    }
    Ok((jar.remove(SESSION_ID), Redirect::to("/")))
}

fn set_cookie(
    jar: CookieJar,
    name: &'static str,
    value: String,
    max_age: i64,
    prod: bool,
) -> CookieJar {
    let mut cookie = Cookie::build((name, value))
        .path("/")
        .http_only(true)
        .secure(prod)
        .same_site(if prod { SameSite::None } else { SameSite::Lax })
        .max_age(time::Duration::seconds(max_age));
    if prod {
        cookie = cookie.domain(".lambda.new")
    }
    jar.add(cookie)
}

fn check_csrf_token(jar: &CookieJar, state: &str) -> anyhow::Result<(), ApiError> {
    let stored_state = jar
        .get(CSRF_TOKEN)
        .ok_or_else(|| ApiError::Unauthorized)?
        .value();
    if stored_state != state {
        return Err(ApiError::Unauthorized);
    }
    Ok(())
}

async fn set_user(
    provider: &str,
    provider_id: String,
    email: String,
    conn: &mut bb8::PooledConnection<'static, redis::Client>,
) -> anyhow::Result<String> {
    let provider_key = format!("{provider}:{provider_id}");
    let email_key = format!("email:{email}");
    let id = if let Some(id) = conn.get(&provider_key).await? {
        id
    } else if let Some(id) = conn.get(&email_key).await? {
        id
    } else {
        uuid::Uuid::now_v7().to_string()
    };
    let user = User {
        id: id.clone(),
        email,
    };
    conn.set::<_, _, ()>(format!("user:{id}"), serde_json::to_string(&user)?)
        .await?;
    conn.set::<_, _, ()>(provider_key, &id).await?;
    conn.set::<_, _, ()>(email_key, &id).await?;
    Ok(id)
}

async fn set_session(
    conn: &mut bb8::PooledConnection<'static, redis::Client>,
    user_id: String,
) -> anyhow::Result<String> {
    let id = uuid::Uuid::now_v7().to_string();
    conn.set_ex::<_, _, ()>(format!("session:{id}"), user_id, SESSION_ID_MAX_AGE as u64)
        .await?;
    Ok(id)
}
