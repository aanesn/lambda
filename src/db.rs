use crate::{Ctx, error::ApiError};
use anyhow::Context;
use axum::{
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};

pub type ConnectionPool = bb8::Pool<redis::Client>;

pub struct DatabaseConnection(pub bb8::PooledConnection<'static, redis::Client>);

impl<S> FromRequestParts<S> for DatabaseConnection
where
    ConnectionPool: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = ApiError;
    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = ConnectionPool::from_ref(state);
        let conn = pool
            .get_owned()
            .await
            .context("failed to get db connection")?;
        Ok(Self(conn))
    }
}

impl FromRef<Ctx> for ConnectionPool {
    fn from_ref(ctx: &Ctx) -> Self {
        ctx.db.clone()
    }
}
