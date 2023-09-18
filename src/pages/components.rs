use anyhow::Result;
use maud::{html, Markup};
use sqlx::SqlitePool;

use crate::db;

pub async fn post_details(post_id: i64, pool: &SqlitePool) -> Result<Markup> {
    let post = db::get_post(post_id, pool).await?;
    let top_note = db::get_top_note(post_id, pool).await?;
    let top_note_id = top_note.clone().map(|post| post.id);
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
            (vote_form(post.id, top_note_id))
        }
    })
}

pub fn vote_form(post_id: i64, note_id: Option<i64>) -> Markup {
    html! {
        form form id="form" hx-post="/vote" hx-trigger="click" {
            input type="hidden" value=(post_id) name="post_id";
            @if let Some(note_id) = note_id {
                input type="hidden" value=(note_id) name="note_id";
            }
            button
                class=""
                name="direction"
                value="Up"
            {
                "▲"
            }

            button
                class=""
                name="direction"
                value="Down"
            {
                "▼"
            }

        }
    }
}
