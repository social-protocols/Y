use anyhow::Result;
use axum::{extract::Path, Extension};
use maud::{html, Markup};
use sqlx::SqlitePool;

use crate::db;
use crate::pages::components::{post_details, vote_form};
use crate::structs::Direction;
use crate::structs::Post;
use crate::{error::AppError, structs::User};

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
                (post_details(&post, maybe_user_id, &pool).await?)
                (reply_form(post_id))
                (replies(post_id, maybe_user_id, &pool).await?)
            }
        }
        None => html! { "Post not found" },
    };
    Ok(base.title("Y").content(content).render())
}

async fn parent_thread(
    post: &Post,
    maybe_user_id: Option<i64>,
    pool: &SqlitePool,
) -> Result<Markup> {
    let transitive_parents: Vec<Post> = db::get_transitive_parents(post.id, pool).await?;

    Ok(html! {
        @for parent in transitive_parents.iter().rev() {
            a href=(format!("/view_post/{}", parent.id)) {
                div class="truncate mb-2 p-3 rounded-lg shadow bg-gray-80 dark:bg-slate-600 ml-4" {
                    (parent.content)
                    ({
                        let current_parent_vote =  match maybe_user_id {
                            Some(user_id) => db::get_current_vote(parent.id, user_id, pool).await?,
                            None => Direction::None
                        };
                        vote_form(parent.id, Some(post.id), current_parent_vote)
                    })
                }
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
                    class="p-5 resize-none w-full text-black"
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

async fn replies(post_id: i64, maybe_user_id: Option<i64>, pool: &SqlitePool) -> Result<Markup> {
    let replies = db::get_replies(post_id, pool).await?;

    let current_vote = match maybe_user_id {
        None => Direction::None,
        Some(id) => db::get_current_vote(post_id, id, pool).await?,
    };

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
                        vote_form(post.id, top_note.map(|post| post.id), current_vote)
                    })
                }
            }
        }
    })
}
