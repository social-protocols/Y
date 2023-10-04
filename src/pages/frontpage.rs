use crate::error::AppError;

use crate::{db, pages::components::post_details, pages::positions::load_positions_js_for_homepage};
use common::structs::User;

use anyhow::Result;
use axum::Extension;
use maud::{html, Markup};

use sqlx::SqlitePool;

use crate::pages::base_template::BaseTemplate;

pub async fn frontpage(
    _maybe_user: Option<User>,
    Extension(_pool): Extension<SqlitePool>,
    base: BaseTemplate,
) -> Result<Markup, AppError> {

    let content = html! {
        div class="mb-10" {
            div {
                (create_post_form())
                (posts(&_pool).await?)
                (load_positions_js_for_homepage())
            }
        }
    };
    Ok(base.title("Y").content(content).render())
}

fn create_post_form() -> Markup {
    html! {
        div class="bg-white rounded-lg shadow-lg w-120 h-30 p-5 mb-10 flex" {
            form hx-post="/create_post" {
                div class="w-full flex" {
                    div class="mr-1" {
                        textarea
                            name="post_content"
                            // class="p-10 resize-none w-full text-black"
                            class="block p-2.5 text-gray-900 bg-gray-50 rounded-lg border border-gray-300 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" cols="100" rows="1"
                            style="width: 100%"
                            placeholder="New Post" {}
                    }
                    button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded float-right tx-sm" {
                        "Post"
                    }
                }
            }
        }
    }
}

async fn posts(pool: &SqlitePool) -> Result<Markup> {
    let posts = db::list_top_level_posts(pool).await?;
    Ok(html! {
        div {
            @for post in posts.iter() {
                div {
                    (post_details(post, false, pool).await?)
                }
            }
        }
    })
}
