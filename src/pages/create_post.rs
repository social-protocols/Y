use crate::{db, error::AppError};
use anyhow::anyhow;
use axum::{extract::Query, response::IntoResponse, Extension, Form};
use common::auth;
use http::StatusCode;
use serde::Deserialize;
use sqlx::SqlitePool;
use tower_cookies::Cookies;

fn default_none<T>() -> Option<T> {
    None
}

#[derive(Deserialize)]
pub struct CreatePostForm {
    post_content: String,
    tag: String,
    #[serde(default = "default_none")]
    post_parent_id: Option<i64>,
}

#[derive(Deserialize)]
pub struct Redirect {
    #[serde(default = "default_none")]
    redirect: Option<String>,
}

pub async fn create_post(
    redirect: Query<Redirect>,
    cookies: Cookies,
    Extension(pool): Extension<SqlitePool>,
    Form(form_data): Form<CreatePostForm>,
) -> Result<impl IntoResponse, AppError> {
    let user = auth::get_or_create_user(&cookies, &pool).await?;
    if form_data.post_content.is_empty() {
        return Err(AppError(anyhow!("Post content cannot be empty")));
    }
    let tag = form_data.tag;

    let _post_id = db::create_post(
        tag.as_str(),
        form_data.post_parent_id,
        form_data.post_content.as_str(),
        user.id,
        &pool,
    )
    .await?;

    let redirect_url = redirect.0.redirect.unwrap_or_else(|| "/".to_string());

    Ok((StatusCode::OK, [("HX-Location", redirect_url)]))
}
