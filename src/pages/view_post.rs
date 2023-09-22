use anyhow::Result;
use async_recursion::async_recursion;
use axum::{extract::Path, Extension};
use maud::{html, Markup};
use sqlx::SqlitePool;

use crate::db;
use crate::pages::components::{post_details, show_current_vote, note};
use crate::structs::Post;
use crate::structs::Direction;
use crate::{error::AppError, structs::User};

use super::base_template::BaseTemplate;

pub async fn view_post(
    Path(post_id): Path<i64>,
    maybe_user: Option<User>,
    Extension(pool): Extension<SqlitePool>,
    base: BaseTemplate,
) -> Result<Markup, AppError> {
    let post = db::get_post(post_id, &pool).await?;

    let maybe_user_id = maybe_user.map(|u| u.id);
   
    let content = html! {
        (parent_thread(&post, maybe_user_id, &pool).await?)
        (post_details(post_id, maybe_user_id, true, &pool).await?)
        (replies(post.id, maybe_user_id, &pool).await?)
    };
    Ok(base.title("Y").content(content).render())
}

#[async_recursion]
async fn parent_thread(post: &Post, maybe_user_id: Option<i64>, pool: &SqlitePool) -> Result<Markup> {

    if post.parent_id.is_none() {
        Ok(html! { div {} })
    } else {
        let parent_id = post.parent_id.unwrap();

        let current_parent_vote = match maybe_user_id {
            None => Direction::None,
            Some(id) => db::get_current_vote(parent_id, id, pool).await?,
        };

        Ok(html! {
            @let parent = db::get_post(parent_id, pool).await?;
            (parent_thread(&parent, maybe_user_id, pool).await?)

            a href=(format!("/view_post/{}", parent_id)) {
                div class="mb-2 p-3 rounded-lg shadow bg-gray-80 dark:bg-slate-600 ml-4" {
                    div class="truncate" { (parent.content) }
                    (show_current_vote(current_parent_vote))
                }
            }
        })    
    }
}


async fn replies(post_id: i64, maybe_user_id: Option<i64>, pool: &SqlitePool) -> Result<Markup> {
    // let top_note = db::get_top_note(post_id, pool).await?;
    // let top_note_id = top_note.map(|n| n.id);

    let replies: Vec<Post> = db::list_replies(post_id, pool).await?;

    // let filtered_replies: Vec<&Post> = replies
    //     .iter()
    //     .filter(|r| match top_note_id { None => true, Some(id) => r.id != id} )
    //     .collect();

    Ok(html! {
        div {
            @for post in replies[1..].iter() {
                (note(Some(post.clone()), Direction::None))
            }
        }
    })
}
