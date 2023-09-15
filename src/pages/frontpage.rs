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
            "Hello world!"
        }
    };
    Ok(base.title("Y").content(content).render())
}
