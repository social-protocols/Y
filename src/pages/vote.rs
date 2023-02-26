use crate::{
    db::UserQueries, error::Error, next_statement::redirect_to_next_statement, auth::User,
};

use axum::{response::Redirect, Extension, Form};
use serde::Deserialize;
use sqlx::SqlitePool;
use tower_cookies::Cookies;

#[derive(Deserialize)]
pub struct VoteForm {
    statement_id: i64,
    vote: i32,
}

pub async fn vote(
    cookies: Cookies,
    Extension(pool): Extension<SqlitePool>,
    Form(vote): Form<VoteForm>,
) -> Result<Redirect, Error> {
    let user = User::get_or_create(&cookies, &pool).await?;

    user.vote(vote.statement_id, vote.vote, &pool).await?;

    Ok(redirect_to_next_statement(Some(user), Extension(pool)).await)
}
