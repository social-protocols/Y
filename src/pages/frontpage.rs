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
            div {
                (create_post_form())
                (posts(&_pool).await?)
                (load_positions_js_for_homepage())
            }
        }
    };
    Ok(base.title("Y").content(content).render())
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
