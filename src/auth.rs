use crate::{Ctx, error::AppError};
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
const CSRF_TOKEN_MAX_AGE: i64 = 600;

pub fn mount() -> Router<Ctx> {
    Router::new()
        .route("/google", get(google))
        .route("/github", get(github))
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
        .add_scope(Scope::new("user:email".to_owned()))
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
