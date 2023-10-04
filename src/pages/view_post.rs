use anyhow::Result;
use axum::{extract::Path, Extension};
use maud::{html, Markup};
use sqlx::SqlitePool;

use crate::db;

use crate::pages::components::{post_details, vote_class};
use common::structs::{Direction, Post, User};
use crate::{error::AppError};

use super::base_template::BaseTemplate;

pub async fn view_post(
    Path(post_id): Path<i64>,
    maybe_user: Option<User>,
    Extension(pool): Extension<SqlitePool>,
    base: BaseTemplate,
) -> Result<Markup, AppError> {
    let post = db::get_post(post_id, &pool).await?;
    let content = match post {
        Some(post) => {
            let maybe_user_id = maybe_user.map(|u| u.id);

            html! {
                (parent_thread(&post, maybe_user_id, &pool).await?)
                (post_details(&post, maybe_user_id, true, &pool).await?)
                (replies(post_id, maybe_user_id, &pool).await?)
            }
        }
        None => html! { "Post not found" },
    };
    Ok(base.title("𝕐").content(content).render())
}




async fn parent_thread(
    post: &Post,
    maybe_user_id: Option<i64>,
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

                    let current_vote = match maybe_user_id {
                        None => Direction::Neutral,
                        Some(id) => db::get_current_vote(parent.id, id, pool).await?,
                    };

                    html! {
                        div class=(format!("truncate mb-2 p-3 rounded-lg shadow ml-4 bg-gray-100 dark:bg-slate-600 {}", vote_class(current_vote))) {
                            (parent.content)
                            
                        }
                    }

                })
            }
        }
    })
}


async fn replies(post_id: i64, _maybe_user_id: Option<i64>, pool: &SqlitePool) -> Result<Markup> {
    let replies = db::get_replies(post_id, pool).await?;

    Ok(html! {
        div {
            @if replies.len() > 1 { 
                h2 class="mt-4 ml-2 mb-2" { "All Replies" }
                @for post in replies.iter() {
                    div class="mb-5 p-5 rounded-lg shadow bg-white dark:bg-slate-700" {
                        a href=(format!("/view_post/{}", post.id)) {
                            div {
                                (post.content)
                            }
                        }
                        // ({
                        //     let top_note = db::get_top_note(post.id, pool).await?;
                        //     vote_form(post.id, top_note.map(|post| post.id), current_vote)
                        // })
                    }
                }
            }
        }
    })
}
