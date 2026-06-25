use crate::Ctx;
use axum::{
    Router,
    extract::State,
    response::{IntoResponse, Redirect},
    routing::get,
};
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};
use oauth2::{CsrfToken, Scope};

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

async fn github_callback() -> impl IntoResponse {
    "test".to_string()
}

async fn google_callback() -> impl IntoResponse {
    "test".to_string()
}
