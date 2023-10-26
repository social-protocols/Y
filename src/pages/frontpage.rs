use crate::error::AppError;

use crate::{
    db, pages::components::create_post_form, pages::components::post_details,
    pages::positions::load_positions_js_for_homepage,
};
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
            div class="fixed top-0 left-0 m-5" {
                (most_used_tags(&_pool).await?)
            }
            div {
                (create_post_form())
                (posts(&_pool).await?)
                (load_positions_js_for_homepage())
            }
        }
    };
    Ok(base.title("Y").content(content).render())
}

async fn most_used_tags(pool: &SqlitePool) -> Result<Markup, AppError> {
    let tags = db::get_top_5_tags(pool).await?;
    Ok(html! {
        ul class="list-none" {
            @for tag in tags.iter() {
                li class="font-bold pb-4" {
                    a
                        href=(format!("/s/{tag}"))
                    {
                        (format!("#{tag}"))
                    }
                }
            }
        }
    })
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
