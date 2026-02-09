use crate::{
    Ctx,
    db::{Connection, DatabaseConnection},
    error::AppError,
    user::User,
};
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
use redis::AsyncCommands;
use serde::Deserialize;

const CSRF_TOKEN: &str = "csrf_token";
const CSRF_TOKEN_MAX_AGE: i64 = 600;
pub const SESSION: &str = "session";
const SESSION_MAX_AGE: i64 = 7 * 24 * 60 * 60; // 7 days

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
) -> anyhow::Result<impl IntoResponse, AppError> {
    let (auth_url, csrf_token) = ctx
        .google
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("profile".to_owned()))
        .add_scope(Scope::new("email".to_owned()))
        .url();
    Ok((
        set_cookie(
            CSRF_TOKEN,
            csrf_token.secret().to_owned(),
            CSRF_TOKEN_MAX_AGE,
            ctx.prod,
            jar,
        ),
        Redirect::to(auth_url.as_ref()),
    ))
}

pub async fn github(
    State(ctx): State<Ctx>,
    jar: CookieJar,
) -> anyhow::Result<impl IntoResponse, AppError> {
    let (auth_url, csrf_token) = ctx
        .github
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("user".to_owned()))
        .url();
    Ok((
        set_cookie(
            CSRF_TOKEN,
            csrf_token.secret().to_owned(),
            CSRF_TOKEN_MAX_AGE,
            ctx.prod,
            jar,
        ),
        Redirect::to(auth_url.as_ref()),
    ))
}

#[derive(Deserialize)]
pub struct CallbackParams {
    code: String,
    state: String,
}

#[derive(Deserialize)]
struct GoogleUser {
    sub: String,
    email: String,
    email_verified: bool,
    name: Option<String>,
    picture: Option<String>,
}

pub async fn google_callback(
    jar: CookieJar,
    Query(params): Query<CallbackParams>,
    State(ctx): State<Ctx>,
    DatabaseConnection(mut conn): DatabaseConnection,
) -> anyhow::Result<impl IntoResponse, AppError> {
    check_csrf_token(&jar, &params.state)?;

    let token = ctx
        .google
        .exchange_code(AuthorizationCode::new(params.code.clone()))
        .request_async(&ctx.reqwest)
        .await
        .context("failed to get google access token")?;

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
        return Err(AppError::Unauthorized);
    }

    let user_id = upsert_user(
        "google",
        &user.sub,
        user.email,
        &mut conn,
        user.name,
        user.picture,
    )
    .await?;

    let session_id = create_session(&mut conn, user_id).await?;

    Ok((
        set_cookie(SESSION, session_id, SESSION_MAX_AGE, ctx.prod, jar),
        Redirect::to(&format!("{}/dashboard/new-project", ctx.client_url)),
    ))
}

#[derive(Deserialize)]
struct GithubUser {
    login: String,
    id: u64,
    name: Option<String>,
    avatar_url: Option<String>,
}

#[derive(Deserialize)]
struct GithubEmail {
    email: String,
    verified: bool,
    primary: bool,
}

pub async fn github_callback(
    jar: CookieJar,
    Query(params): Query<CallbackParams>,
    State(ctx): State<Ctx>,
    DatabaseConnection(mut conn): DatabaseConnection,
) -> anyhow::Result<impl IntoResponse, AppError> {
    check_csrf_token(&jar, &params.state)?;

    let token = ctx
        .github
        .exchange_code(AuthorizationCode::new(params.code.clone()))
        .request_async(&ctx.reqwest)
        .await
        .context("failed to get github access token")?;

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
        .find(|e| e.primary && e.verified)
        .map(|e| e.email)
        .ok_or(AppError::Unauthorized)?;

    let user_id = upsert_user(
        "github",
        &user.id.to_string(),
        email,
        &mut conn,
        user.name.or(Some(user.login)),
        user.avatar_url,
    )
    .await?;

    let session_id = create_session(&mut conn, user_id).await?;

    Ok((
        set_cookie(SESSION, session_id, SESSION_MAX_AGE, ctx.prod, jar),
        Redirect::to(&format!("{}/dashboard/new-project", ctx.client_url)),
    ))
}

pub async fn logout(
    jar: CookieJar,
    DatabaseConnection(mut conn): DatabaseConnection,
    State(ctx): State<Ctx>,
) -> anyhow::Result<impl IntoResponse, AppError> {
    if let Some(session_id) = jar.get(SESSION).map(|c| c.value()) {
        conn.del::<String, ()>(format!("session:{session_id}"))
            .await?;
    }
    Ok((
        delete_cookie(SESSION, ctx.prod, jar),
        Redirect::to(&ctx.client_url),
    ))
}

fn set_cookie(
    name: &'static str,
    value: String,
    max_age: i64,
    prod: bool,
    jar: CookieJar,
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

fn delete_cookie(name: &'static str, prod: bool, jar: CookieJar) -> CookieJar {
    let mut cookie = Cookie::build(name)
        .path("/")
        .http_only(true)
        .secure(prod)
        .same_site(if prod { SameSite::None } else { SameSite::Lax });
    if prod {
        cookie = cookie.domain(".lambda.new")
    }
    jar.remove(cookie)
}

fn check_csrf_token(jar: &CookieJar, state: &str) -> Result<(), AppError> {
    jar.get(CSRF_TOKEN)
        .filter(|c| c.value() == state)
        .ok_or(AppError::Unauthorized)?;
    Ok(())
}

async fn upsert_user(
    provider: &str,
    provider_id: &str,
    email: String,
    conn: &mut Connection,
    name: Option<String>,
    avatar_url: Option<String>,
) -> anyhow::Result<String> {
    let provider_key = format!("user:{provider}:{provider_id}");
    let email_key = format!("user:email:{email}");
    let user_id = if let Some(user_id) = conn.get(&provider_key).await? {
        user_id
    } else if let Some(user_id) = conn.get(&email_key).await? {
        user_id
    } else {
        uuid::Uuid::now_v7().to_string()
    };
    let user = User {
        id: user_id.clone(),
        email,
        name,
        avatar_url,
    };
    conn.set::<String, String, ()>(format!("user:{user_id}"), serde_json::to_string(&user)?)
        .await?;
    conn.set::<&str, &str, ()>(&provider_key, &user_id).await?;
    conn.set::<&str, &str, ()>(&email_key, &user_id).await?;
    Ok(user_id)
}

async fn create_session(conn: &mut Connection, user_id: String) -> anyhow::Result<String> {
    let session_id = uuid::Uuid::now_v7().to_string();
    conn.set_ex::<String, String, ()>(
        format!("session:{session_id}"),
        user_id,
        SESSION_MAX_AGE as u64,
    )
    .await?;
    Ok(session_id)
}
