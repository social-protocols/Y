use crate::db;
use crate::error::AppError;
use anyhow::Result;
use axum::Extension;
use axum::{response::IntoResponse, Form};
use http::StatusCode;
use serde::Deserialize;
use sqlx::SqlitePool;

#[derive(Deserialize, Debug)]
pub struct HashtagRequest {
    post_id: i64,
    hashtag: String,
}

pub async fn add_hashtag(
    Extension(pool): Extension<SqlitePool>,
    Form(form_data): Form<HashtagRequest>,
) -> Result<impl IntoResponse, AppError> {
    db::add_hashtag(form_data.post_id, form_data.hashtag.as_str(), &pool).await?;
    Ok(StatusCode::OK)
}

