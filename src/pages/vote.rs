use axum::{Extension, Form};
use maud::{html, Markup};
use sqlx::SqlitePool;
use tower_cookies::Cookies;

use crate::db;
use crate::error::AppError;
use serde::Deserialize;

use crate::structs::Direction;
use crate::structs::User;

fn default_none() -> Option<i64> {
    None
}

#[derive(Deserialize)]
pub struct VoteRequest {
    post_id: i64,
    #[serde(default = "default_none")]
    note_id: Option<i64>,
    direction: Direction,
}

pub async fn vote(
    cookies: Cookies,
    Extension(pool): Extension<SqlitePool>,
    Form(form_data): Form<VoteRequest>,
) -> Result<Markup, AppError> {
    let user = User::get_or_create(&cookies, &pool).await?;
    db::vote(
        user.id,
        form_data.post_id,
        form_data.note_id,
        form_data.direction,
        &pool,
    )
    .await?;

    Ok(html! {"voted"})
}
