use std::num::ParseIntError;

use anyhow::Result;
use axum::{extract::Path, Extension};
use maud::{html, Markup};
use sqlx::SqlitePool;

use crate::db;
use crate::{error::AppError, structs::User};

use super::base_template::BaseTemplate;

pub async fn view_post(
    Path(post_id): Path<i64>,
    _maybe_user: Option<User>,
    Extension(pool): Extension<SqlitePool>,
    base: BaseTemplate,
) -> Result<Markup, AppError> {
    let post = db::get_post(post_id, &pool).await?;
    let content = html! {
        div class="mb-5 p-5 rounded-lg shadow bg-white dark:bg-slate-700" {
            div {
                (post.content)
            }
            div {
                (top_note(post.id, &pool).await?)
            }
        }
        div class="mb-5 p-5 rounded-lg shadow bg-white dark:bg-slate-700" {
            div {
                (create_reply_form(post_id))
            }
        }
        (replies(post.id, &pool).await?)
    };
    Ok(base.title("Y").content(content).render())
}

fn create_reply_form(parent_id: i64) -> Markup {
    html! {
        div class="bg-white rounded-lg shadow-lg w-120 h-30 p-5 mb-10" {
            form hx-post="/create_post" {
                input type="hidden" name="post_parent_id" value=(format!("{}", parent_id)) {}
                textarea name="post_content" class="p-10 resize-none w-full" placeholder="Enter your reply" {
                }
                div class="flex justify-end" {
                    button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded float-none" {
                        "Reply"
                    }
                }
            }
        }
    }
}

async fn top_note(post_id: i64, pool: &SqlitePool) -> Result<Markup> {
    let top_note = db::get_top_note(post_id, pool).await?;
    Ok(html! {
        @match top_note {
            Some(post) => div class="mb-5 p-5 rounded-lg shadow bg-white dark:bg-slate-700" { (post.content) },
            None => div {},
        }
    })
}

async fn replies(post_id: i64, pool: &SqlitePool) -> Result<Markup> {
    let replies = db::list_replies(post_id, pool).await?;
    Ok(html! {
        div {
            @for post in replies.iter() {
                div class="mb-5 p-5 rounded-lg shadow bg-white dark:bg-slate-700" {
                    a href=(format!("/view_post/{}", post.id)) {
                        div {
                            (post.content)
                        }
                    }

                    form form id="form" hx-post="/vote" hx-trigger="click" hx-swap="none" {
                        input type="hidden" value=(post.id) name="post_id";

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
        }
    })
}
