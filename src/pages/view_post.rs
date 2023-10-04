use anyhow::Result;
use axum::{extract::Path, Extension};
use maud::{html, Markup};
use sqlx::SqlitePool;

use crate::db;

use crate::pages::components::{post_details};
use crate::pages::positions::load_positions_js;

use common::structs::{Post, User};
use crate::{error::AppError};

use super::base_template::BaseTemplate;

pub async fn view_post(
    Path(post_id): Path<i64>,
    _maybe_user: Option<User>,
    Extension(pool): Extension<SqlitePool>,
    base: BaseTemplate,
) -> Result<Markup, AppError> {
    let post = db::get_post(post_id, &pool).await?;
    let content = match post {
        Some(post) => {
            html! {
                (parent_thread(&post, &pool).await?)
                (post_details(&post, true, &pool).await?)
                (replies(post_id, &pool).await?)
                (load_positions_js(post_id))
            }
        }
        None => html! { "Post not found" },
    };
    Ok(base.title("𝕐").content(content).render())
}




async fn parent_thread(
    post: &Post,
    pool: &SqlitePool,
) -> Result<Markup> {
    let transitive_parents: Vec<Post> = db::get_transitive_parents(post, pool).await?;
    Ok(html! {
        a href="/" {
            div class="truncate mb-2 p-3 rounded-lg shadow bg-gray-100 dark:bg-slate-600 ml-4" {
                "𝕐 home"
            }
        }

        @for parent in transitive_parents.iter().rev() {
            a href=(format!("/view_post/{}", parent.id)) {

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

async fn replies(post_id: i64, pool: &SqlitePool) -> Result<Markup> {
    let replies = db::get_replies(post_id, pool).await?;

    Ok(html! {
        div {
            @if replies.len() > 1 { 
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
