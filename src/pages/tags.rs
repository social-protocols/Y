use crate::db;
use crate::error::AppError;
use anyhow::Result;
use axum::Extension;
use axum::{response::IntoResponse, Form};
use http::StatusCode;
use serde::Deserialize;
use sqlx::SqlitePool;

#[derive(Deserialize, Debug)]
pub struct TagRequest {
    post_id: i64,
    tag: String,
}

pub async fn add_tag(
    Extension(pool): Extension<SqlitePool>,
    Form(form_data): Form<TagRequest>,
) -> Result<impl IntoResponse, AppError> {
    db::add_tag(form_data.post_id, form_data.tag.as_str(), &pool).await?;
    Ok(StatusCode::OK)
}
