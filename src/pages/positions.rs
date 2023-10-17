use axum::{Extension, Form};
// use common::auth;
use maud::{html, Markup, PreEscaped};
use sqlx::{SqlitePool};
use tower_cookies::Cookies;

// use crate::db;
use crate::error::AppError;
use serde::{Deserialize};
use serde_json;

use anyhow::Result;
use common::auth::get_or_create_user;

use common::structs::{User};


#[derive(Deserialize)]
pub struct PositionsRequest {
    post_id: i64
}

pub async fn positions(
    cookies: Cookies,
    Extension(pool): Extension<SqlitePool>,
    Form(form_data): Form<PositionsRequest>,
) -> Result<Markup, AppError> {

    let user = get_or_create_user(&cookies, &pool).await?;

    let user_id = user.id;

    let positions: Vec<(i64, i64)> = if form_data.post_id == 0 {
        get_positions_for_frontpage(user_id, &pool).await?
    } else {
        get_positions_for_post(form_data.post_id, user_id, &pool).await?
    };

    let json = serde_json::to_string(&positions)?;

    Ok(html! {
        script language="javascript" {
            "var userID = " (user_id) ";"
            "var positions = " (PreEscaped(json)) ";"
            "setPositions(userID, positions);"
        }
    })
}


pub async fn get_positions_for_post(post_id: i64, user_id: i64, pool: &SqlitePool) -> Result<Vec<(i64, i64)>> {

    let query = r#"
        WITH ancestors AS
        (
          SELECT id, parent_id
          FROM posts
          WHERE id = ?
          UNION ALL
          SELECT p.id, p.parent_id
          FROM ancestors a
          INNER JOIN posts p ON a.parent_id = p.id
        )
        , children as (
          select id from posts where parent_id = ?
        )
        SELECT post_id, direction
        FROM (select id from ancestors UNION ALL select id from children) ids
        join current_vote on (post_id = id)
        where user_id = ?
    "#;

    // execute the query and get a vector of Votes
    let positions: Vec<(i64, i64)> = sqlx::query_as::<_, (i64, i64)>(query)
        .bind(post_id)
        .bind(post_id)
        .bind(user_id)
        .fetch_all(pool)
        .await?;

    Ok(positions)
}


pub async fn get_positions_for_frontpage(user_id: i64, pool: &SqlitePool) -> Result<Vec<(i64, i64)>> {

    let query = r#"
        select 
            post_id, direction
        from 
            current_vote 
            join posts on (post_id = id)
        where 
            user_id = ?
            and posts.parent_id is null
    "#;

    // execute the query and get a vector of Votes
    let positions: Vec<(i64, i64)> = sqlx::query_as::<_, (i64, i64)>(query)
        .bind(user_id)
        .fetch_all(pool)
        .await?;

    Ok(positions)
}

pub fn load_positions_js_for_homepage() -> Markup {
    // kinda hacky. In future, maybe there is an argument for a tag, community id, etc.
    load_positions_js(0)
}

pub fn load_positions_js(post_id: i64) -> Markup {
    html! {
        form hx-trigger="load" hx-get="/positions" {
            input type="hidden" name="post_id" value=(post_id) {}
        }
    }
}

