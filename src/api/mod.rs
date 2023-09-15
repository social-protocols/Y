use anyhow::Result;

use axum::Extension;

use sqlx::SqlitePool;

use crate::{error::AppError, structs::User};

pub async fn create_user(Extension(pool): Extension<SqlitePool>) -> Result<String, AppError> {
    let user = User::create(&pool).await?;
    Ok(user.secret)
}
