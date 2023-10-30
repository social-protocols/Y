use axum::{Extension, Form};
use common::auth;
use maud::{html, Markup};
use regex;
use sqlx::SqlitePool;
use tower_cookies::Cookies;

use crate::db;
use crate::error::AppError;
use crate::pages::components::tag_form;
use regex::Regex;
use serde::Deserialize;

use anyhow::Result;

use common::structs::Direction;
use common::structs::Direction::Neutral;

fn default_none() -> Option<i64> {
    None
}

#[derive(Deserialize)]
pub struct VoteRequest {
    tag: String,
    post_id: i64,
    #[serde(default = "default_none")]
    note_id: Option<i64>,
    direction: Direction,
    state: Direction,
}

#[derive(Deserialize)]
pub struct TagRequest {
    tags: String,
    post_id: i64,
    #[serde(default = "default_none")]
    note_id: Option<i64>,
}

pub async fn vote_handler(
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

    let user = auth::get_or_create_user(&cookies, &pool).await?;
    db::vote(
        user.id,
        form_data.tag.as_str(),
        form_data.post_id,
        form_data.note_id,
        new_state,
        &pool,
    )
    .await?;

    Ok(vote_buttons(
        form_data.tag.as_str(),
        form_data.post_id,
        form_data.note_id,
        new_state,
    ))
}

pub fn vote_buttons(tag: &str, post_id: i64, note_id: Option<i64>, state: Direction) -> Markup {
    html! {
        div class="vote-buttons mt-2 w-7" {
            input type="hidden" value=(post_id) name="post_id";
            input type="hidden" value=(state) name="state";
            input type="hidden" value=(tag) name="tag";
            @if let Some(note_id) = note_id {
                input type="hidden" value=(note_id) name="note_id";
            }
            button
                class="upvote"
                name="direction"
                value="Up"
            {
                "▲"
            }

            button
                class="downvote"
                name="direction"
                value="Down"
            {
                "▼"
            }
        }

        script language="javascript" {
            (format!("setPosition({}, {});", post_id, state as i64))
        }
    }
}

pub async fn tag_handler(
    cookies: Cookies,
    Extension(pool): Extension<SqlitePool>,
    Form(form_data): Form<TagRequest>,
) -> Result<Markup, AppError> {
    let user = auth::get_or_create_user(&cookies, &pool).await?;
    let re = Regex::new(r"[^\p{L}]+").unwrap(); // This regex matches one or more commas or spaces
    let tags = re.split(form_data.tags.as_str());
    for tag in tags {
        db::vote(
            user.id,
            tag,
            form_data.post_id,
            form_data.note_id,
            Direction::Up,
            &pool,
        )
        .await?;
    }
    Ok(tag_form(form_data.post_id, form_data.note_id))
}
