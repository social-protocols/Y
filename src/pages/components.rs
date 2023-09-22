use anyhow::Result;
use maud::{html, Markup};
use sqlx::SqlitePool;

use crate::db;

use crate::structs::{Direction, Post};

use crate::pages::vote::vote_buttons;

pub async fn post_details(post: &Post, user_id: Option<i64>, pool: &SqlitePool) -> Result<Markup> {
    let top_note = db::get_top_note(post.id, pool).await?;
    let top_note_id = top_note.clone().map(|post| post.id);
    let current_vote = match user_id {
        None => Direction::None,
        Some(user_id) => db::get_current_vote(post.id, user_id, pool).await?,
    };

    Ok(html! {
        div class="mb-5 p-5 rounded-lg shadow bg-white dark:bg-slate-700" {
            div class="mb-5" {
                (post.content)
            }
            div {
                @match top_note.clone() {
                    Some(post) => {
                        a href=(format!("/view_post/{}", post.id)) {
                            p class="mb-5 p-5 rounded-lg shadow bg-white dark:bg-slate-800" { (post.content) }
                        }
                    },
                    None => div {},
                }
            }
            (vote_form(post.id, top_note_id, current_vote))
        }
    })
}

pub fn vote_form(post_id: i64, note_id: Option<i64>, current_vote: Direction) -> Markup {
    // Todo: initial state from DB if this user has voted

    html! {
        form id="form" hx-post="/vote" hx-trigger="click" {
            (vote_buttons(post_id, note_id, current_vote))
        }
    }
}
