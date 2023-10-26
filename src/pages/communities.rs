use crate::db;
use crate::error::AppError;
use crate::pages::base_template::BaseTemplate;
use crate::pages::components::create_post_form;
use crate::pages::components::post_details;
use anyhow::Result;
use axum::{extract::Path, Extension};
use maud::{html, Markup};
use sqlx::SqlitePool;

pub async fn community_frontpage(
    Path(tag): Path<String>,
    Extension(_pool): Extension<SqlitePool>,
    base: BaseTemplate,
) -> Result<Markup, AppError> {
    let content = html! {
        (create_post_form())
        h1 class="text-xl font-bold mb-4" { (format!("#{tag}")) }
        (posts_with_tag(tag.as_str(), &_pool).await?)
    };
    Ok(base.title("Y").content(content).render())
}

async fn posts_with_tag(tag: &str, pool: &SqlitePool) -> Result<Markup> {
    let posts = db::get_top_level_posts_with_tag(tag, pool).await?;
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
