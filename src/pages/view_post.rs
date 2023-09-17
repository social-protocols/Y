use anyhow::Result;
use async_recursion::async_recursion;
use axum::{extract::Path, Extension};
use maud::{html, Markup};
use sqlx::SqlitePool;

use crate::db;
use crate::pages::components::{post_details, vote_form};
use crate::structs::Post;
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
        (parent_thread(&post, &pool).await?)
        (post_details(post_id, &pool).await?)
        (reply_form(post_id))
        (replies(post.id, &pool).await?)
    };
    Ok(base.title("Y").content(content).render())
}

#[async_recursion]
async fn parent_thread(post: &Post, pool: &SqlitePool) -> Result<Markup> {
    Ok(html! {
        @if post.parent_id.is_none() {
            div {}
        } @else {
            @let parent = db::get_post(post.parent_id.unwrap(), pool).await?;
            div {
                (parent_thread(&parent, pool).await?)
                (parent.content)
            }
        }
    })
}

fn reply_form(parent_id: i64) -> Markup {
    html! {
        div class="bg-white rounded-lg shadow-lg w-120 h-30 p-5 mb-10" {
            form hx-post=(format!("/create_post?redirect=/view_post/{}", parent_id)) {
                input
                    type="hidden"
                    name="post_parent_id"
                    value=(format!("{}", parent_id)) {}
                textarea
                    name="post_content"
                    class="p-10 resize-none w-full text-black"
                    placeholder="Enter your reply" {}
                div class="flex justify-end" {
                    button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded float-none" {
                        "Reply"
                    }
                }
            }
        }
    }
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
                    ({
                        let top_note = db::get_top_note(post.id, pool).await?;
                        vote_form(post.id, top_note.map(|post| post.id))
                    })
                }
            }
        }
    })
}
