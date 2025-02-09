use anyhow::Result;
use axum::{extract::Path, Extension};
use maud::{html, Markup};
use sqlx::SqlitePool;

use crate::db;

use crate::pages::components::post_details;
use crate::pages::positions::load_positions_js;

use crate::error::AppError;
use common::structs::{Post, User};

use super::base_template::BaseTemplate;

pub async fn view_post(
    Path((tag_string, post_id)): Path<(String, i64)>,
    _maybe_user: Option<User>,
    Extension(pool): Extension<SqlitePool>,
    base: BaseTemplate,
) -> Result<Markup, AppError> {
    let post = db::get_post(post_id, &pool).await?;
    let tag = tag_string.as_str();
    let content = match post {
        Some(post) => {
            html! {
                (parent_thread(tag, &post, &pool).await?)
                (post_details(tag, &post, true, &pool).await?)
                (replies(tag, post_id, &pool).await?)
                (load_positions_js(tag, post_id))
            }
        }
        None => html! { "Post not found" },
    };
    Ok(base.title("𝕐").content(content).render())
}

async fn parent_thread(tag: &str, post: &Post, pool: &SqlitePool) -> Result<Markup> {
    let transitive_parents: Vec<Post> = db::get_transitive_parents(post, pool).await?;
    Ok(html! {
        a href="/" {
            div class="truncate mb-2 p-3 rounded-lg shadow bg-gray-100 dark:bg-slate-600 ml-4" {
                "𝕐 home"
            }
        }

        @for parent in transitive_parents.iter().rev() {
            a href=(format!("/y/{}/post/{}", tag, parent.id)) {

                ({
                    html! {
                        div data-postid=(parent.id) class="post truncate mb-2 p-3 rounded-lg shadow ml-4 bg-gray-100 dark:bg-slate-600" {
                            (parent.content)
                        }
                    }
                })
            }
        }
    })
}

async fn replies(tag: &str, post_id: i64, pool: &SqlitePool) -> Result<Markup> {
    let replies = db::get_replies(tag, post_id, pool).await?;

    Ok(html! {
        div {
            @if !replies.is_empty() {
                h2 class="mt-4 ml-2 mb-2" { "More Replies" }
                @for post in replies.iter() {
                    div data-postid=(post.id) class="post mb-5 p-5 rounded-lg shadow bg-white dark:bg-slate-700" {
                        a href=(format!("/view_post/{}", post.id)) {
                            div {
                                (post.content)
                            }
                        }
                    }
                }
            }
        }
    })
}
