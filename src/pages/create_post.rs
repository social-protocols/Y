use axum::extract::Query;
use axum::response::IntoResponse;
use axum::{Extension, Form};
use http::StatusCode;
use sqlx::SqlitePool;
use tower_cookies::Cookies;

use anyhow::anyhow;

use crate::db;
use crate::error::AppError;
use serde::Deserialize;

fn default_none<T>() -> Option<T> {
    None
}

#[derive(Deserialize)]
pub struct CreatePostForm {
    post_content: String,
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
    _cookies: Cookies,
    Extension(pool): Extension<SqlitePool>,
    Form(form_data): Form<CreatePostForm>,
) -> Result<impl IntoResponse, AppError> {
    // let user = User::get_or_create(&cookies, &pool).await?;
    if form_data.post_content.is_empty() {
        return Err(AppError(anyhow!("Post content cannot be empty")));
    }
    let _post_id = db::create_post(
        form_data.post_content.as_str(),
        form_data.post_parent_id,
        &pool,
    )
    .await?;

    let redirect_url = redirect.0.redirect.unwrap_or_else(|| "/".to_string());

    Ok((StatusCode::OK, [("HX-Location", redirect_url)]))
}
