use crate::db;
use crate::error::AppError;
use crate::pages::{
    base_template::BaseTemplate,
    components::{create_post_form, post_feed},
};
use anyhow::Result;
use axum::{extract::Path, Extension};
use maud::{html, Markup};
use sqlx::SqlitePool;

pub async fn community_frontpage(
    Path(tag): Path<String>,
    Extension(pool): Extension<SqlitePool>,
    base: BaseTemplate,
) -> Result<Markup, AppError> {
    let posts = db::get_top_level_posts_with_tag(tag.as_str(), &pool).await?;
    let content = html! {
        (create_post_form())
        h1 class="text-xl font-bold mb-4" { (format!("#{tag}")) }
        (post_feed(posts, &pool).await?)
    };
    Ok(base.title("Y").content(content).render())
}
