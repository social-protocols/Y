use crate::error::AppError;
use crate::structs::User;
use crate::{db, pages::components::post_details};

use anyhow::Result;
use axum::Extension;
use maud::{html, Markup};

use sqlx::SqlitePool;

use crate::pages::base_template::BaseTemplate;

pub async fn frontpage(
    maybe_user: Option<User>,
    Extension(_pool): Extension<SqlitePool>,
    base: BaseTemplate,
) -> Result<Markup, AppError> {

    let maybe_user_id = match maybe_user {
        None => None,
        Some(user) => Some(user.id),
    };



    let content = html! {
        div class="mb-10" {
            div {
                (create_post_form())
                (posts(maybe_user_id, &_pool).await?)
            }
        }
    };
    Ok(base.title("Y").content(content).render())
}

fn create_post_form() -> Markup {
    html! {
        div class="bg-white rounded-lg shadow-lg w-120 h-30 p-5 mb-10" {
            form hx-post="/create_post" {
                div class="mr-1" {
                    textarea 
                        name="post_content" 
                        // class="p-10 resize-none w-full text-black" 
                        class="block p-2.5 w-full text-sm text-gray-900 bg-gray-50 rounded-lg border border-gray-300 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" cols="100" rows="1"
                        placeholder="Say something" {}
                }
                div class="justify-end" {
                    button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded float-right" {
                        "Submit"
                    }
                }
            }
        }
    }
}

async fn posts(maybe_user_id: Option<i64>, pool: &SqlitePool) -> Result<Markup> {
    let posts = db::list_top_level_posts(pool).await?;
    Ok(html! {
        div {
            @for post in posts.iter() {
                    (post_details(post.id, maybe_user_id, false, pool).await?)
            }
        }
    })
}
