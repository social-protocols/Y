use crate::{
    db,
    error::AppError,
    pages::{
        base_template::BaseTemplate,
        components::{create_post_form, post_feed},
        positions::load_positions_js_for_tag,
    },
};
use common::structs::User;

use anyhow::Result;
use axum::Extension;
use maud::{html, Markup};

use sqlx::SqlitePool;

use crate::constants::GLOBAL_TAG;

pub async fn frontpage(
    _maybe_user: Option<User>,
    Extension(pool): Extension<SqlitePool>,
    base: BaseTemplate,
) -> Result<Markup, AppError> {
    let posts = db::get_posts_for_tag(GLOBAL_TAG, &pool).await?;
    let content = html! {
        div class="mb-10" {
            div class="fixed top-0 left-0 m-5" {
                (most_used_tags(&pool).await?)
            }
            div {
                (create_post_form())
                (post_feed(GLOBAL_TAG, posts, &pool).await?)
                (load_positions_js_for_tag(GLOBAL_TAG))
            }
        }
    };

    // TODO: redirect from "/y/global"
    Ok(base.title("Y").content(content).render())
}

async fn most_used_tags(pool: &SqlitePool) -> Result<Markup, AppError> {
    let tags = db::get_top_5_tags(pool).await?;
    Ok(html! {
        ul class="list-none" {
            @for tag in tags.iter() {
                li class="font-bold pb-4" {
                    a href=(format!("/s/{tag}")) { (format!("#{tag}")) }
                }
            }
        }
    })
}
