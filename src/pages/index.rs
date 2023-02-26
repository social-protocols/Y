use crate::{auth::User, error::Error, next_statement::redirect_to_next_statement};

use axum::{response::Redirect, Extension};
use sqlx::SqlitePool;

pub async fn index(
    existing_user: Option<User>,
    Extension(pool): Extension<SqlitePool>,
) -> Result<Redirect, Error> {
    Ok(redirect_to_next_statement(existing_user, Extension(pool)).await)
}
