use axum::response::IntoResponse;
use axum::{Extension, Form};
use http::StatusCode;
use sqlx::SqlitePool;
use tower_cookies::Cookies;

use crate::db;
use crate::error::AppError;
use serde::Deserialize;

use crate::structs::Direction;
use crate::structs::User;

#[derive(Deserialize)]
pub struct VoteRequest {
    post_id: i64,
    note_id: Option<i64>,
    direction: Direction,
}

pub async fn vote(
    cookies: Cookies,
    Extension(pool): Extension<SqlitePool>,
    Form(form_data): Form<VoteRequest>,
) -> Result<impl IntoResponse, AppError> {
    println!("vote");
    let user = User::get_or_create(&cookies, &pool).await?;
    db::vote(
        user.id,
        form_data.post_id,
        form_data.note_id,
        form_data.direction,
        &pool,
    )
    .await?;

    Ok((StatusCode::OK, [("HX-Location", "/")]))
}
