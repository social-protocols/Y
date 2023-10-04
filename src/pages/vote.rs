use axum::{Extension, Form};
use common::auth;
use maud::{html, Markup};
use sqlx::SqlitePool;
use tower_cookies::Cookies;

use crate::db;
use crate::error::AppError;
use serde::Deserialize;

use anyhow::Result;


use common::structs::{Direction};
use common::structs::Direction::{Up, Down, Neutral};

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
        Neutral
    } else {
        form_data.direction
    };

    // println!("{:?} {:?} {:?}", form_data.direction, form_data.state, new_state);

    let user = auth::get_or_create_user(&cookies, &pool).await?;
    db::vote(
        user.id,
        form_data.post_id,
        form_data.note_id,
        new_state,
        &pool,
    )
    .await?;

    Ok(vote_buttons(
        form_data.post_id,
        form_data.note_id,
        new_state,
    ))
}

pub fn vote_buttons(post_id: i64, note_id: Option<i64>, state: Direction) -> Markup {
    let upvote_style_class = match state {
        Direction::Up => "text-green-500",
        _ => "",
    };

    let downvote_style_class = match state {
        Direction::Down => "text-red-500",
        _ => "",
    };

    html! {
        span  {
            input type="hidden" value=(post_id) name="post_id";
            input type="hidden" value=(state) name="state";
            @if let Some(note_id) = note_id {
                input type="hidden" value=(note_id) name="note_id";
            }
            button
                class=(format!("upvote {upvote_style_class}"))
                name="direction"
                value="Up"
            {
                "▲"
            }

            button
                class=(format!("downvote {downvote_style_class}"))
                name="direction"
                value="Down"
            {
                "▼"
            }
        }
    }
}
