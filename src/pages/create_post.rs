use axum::response::IntoResponse;
use axum::{Extension, Form};
use http::StatusCode;
use sqlx::SqlitePool;
use tower_cookies::Cookies;

use anyhow::anyhow;

use crate::db;
use crate::error::AppError;
use serde::Deserialize;

fn default_none() -> Option<i64> {
    None
}

#[derive(Deserialize)]
pub struct CreatePostForm {
    post_content: String,
    #[serde(default = "default_none")]
    post_parent_id: Option<i64>,
}

pub async fn create_post(
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

    Ok((StatusCode::OK, [("HX-Location", "/")]))
}
