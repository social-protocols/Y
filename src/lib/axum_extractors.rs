use anyhow::Result;
use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::StatusCode;
use axum::Extension;
use http::request::Parts;
use sqlx::SqlitePool;
use tower_cookies::Cookies;

use crate::auth::user_from_cookies;
use crate::structs::User;

#[async_trait]
impl<S> FromRequestParts<S> for User
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        use axum::RequestPartsExt;
        let Extension(pool) = parts
            .extract::<Extension<SqlitePool>>()
            .await
            .expect("Unable to get sqlite connection");
        let cookies = parts
            .extract::<Cookies>()
            .await
            .expect("Unable to get cookies");

        match user_from_cookies(&cookies, &pool).await {
            Ok(result) => result.ok_or((StatusCode::UNAUTHORIZED, "Unauthorized")),
            Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")),
        }
    }
}
