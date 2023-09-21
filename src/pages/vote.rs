use axum::{Extension, Form};
use maud::{Markup};
use sqlx::SqlitePool;
use tower_cookies::Cookies;

use crate::db;
use crate::error::AppError;
use serde::Deserialize;

use crate::pages::components::vote_buttons;

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
    state: Direction,
}

pub async fn vote(
    cookies: Cookies,
    Extension(pool): Extension<SqlitePool>,
    Form(form_data): Form<VoteRequest>,
) -> Result<Markup, AppError> {

    // First, interpret the user intent based on the button pressed **and** the current state.
    let new_state = if form_data.direction == form_data.state {
        Direction::None
    } else {
        form_data.direction
    };

    // println!("{:?} {:?} {:?}", form_data.direction, form_data.state, new_state);


    let user = User::get_or_create(&cookies, &pool).await?;
    db::vote(
        user.id,
        form_data.post_id,
        form_data.note_id,
        new_state,
        &pool,
    )
    .await?;

    Ok(vote_buttons(form_data.post_id, form_data.note_id, new_state))
}


