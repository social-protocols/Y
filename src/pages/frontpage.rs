use crate::db;
use crate::error::AppError;
use crate::structs::User;

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
        div class="mb-10 flex justify-center" {
            div {
                (create_post_form())
                (posts(&_pool).await?)
            }
        }
    };
    Ok(base.title("Y").content(content).render())
}

fn create_post_form() -> Markup {
    html! {
        div class="bg-white rounded-lg shadow-lg w-120 h-30 p-5 mb-10" {
            form hx-post="/create_post" {
                textarea name="post_content" class="p-10 resize-none w-full" placeholder="Say something" {
                }
                div class="flex justify-end" {
                    button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded" {
                        "Submit"
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
