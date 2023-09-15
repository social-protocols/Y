use axum::response::IntoResponse;
use axum::{Extension, Form};
use http::StatusCode;
use sqlx::SqlitePool;
use tower_cookies::Cookies;

use crate::db;
use crate::error::AppError;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreatePostForm {
    post_content: String,
}

pub async fn create_post(
    _cookies: Cookies,
    Extension(pool): Extension<SqlitePool>,
    Form(form_data): Form<CreatePostForm>,
) -> Result<impl IntoResponse, AppError> {
    println!("create_post");
    // let user = User::get_or_create(&cookies, &pool).await?;
    let _post_id = db::create_post(form_data.post_content.as_str(), &pool).await?;

    Ok((StatusCode::OK, [("HX-Location", "/")]))
}
