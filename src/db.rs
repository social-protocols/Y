//! Database access via sqlx

use anyhow::Result;
use sqlx::SqlitePool;

use crate::structs::Post;

use crate::structs::Direction;

pub async fn create_post(content: &str, parent_id: Option<i64>, pool: &SqlitePool) -> Result<i64> {
    let created_post_id = sqlx::query_scalar::<_, i64>(
        "INSERT INTO posts (content, parent_id) VALUES (?, ?) RETURNING id",
    )
    .bind(content)
    .bind(parent_id)
    .fetch_one(pool)
    .await?;
    Ok(created_post_id)
}

pub async fn get_post(post_id: i64, pool: &SqlitePool) -> Result<Post> {
    let post = sqlx::query_as::<_, Post>("select id, content, parent_id from posts where id = ?")
        .bind(post_id)
        .fetch_one(pool)
        .await?;
    Ok(post)
}

pub async fn vote(
    user_id: i64,
    post_id: i64,
    note_id: Option<i64>,
    direction: Direction,
    pool: &SqlitePool,
) -> Result<()> {
    let direction_i32 = direction as i32;

    sqlx::query(
        "INSERT INTO vote_history (user_id, post_id, note_id, direction) VALUES (?, ?,?,?)",
    )
    .bind(user_id)
    .bind(post_id)
    .bind(note_id)
    .bind(direction_i32)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn list_top_level_posts(pool: &SqlitePool) -> Result<Vec<Post>> {
    let posts = sqlx::query_as::<_, Post>(
        "SELECT id, content, parent_id FROM posts where parent_id is null ORDER BY created DESC",
    )
    .fetch_all(pool)
    .await?;
    Ok(posts)
}

pub async fn list_replies(post_id: i64, pool: &SqlitePool) -> Result<Vec<Post>> {
    let posts = sqlx::query_as::<_, Post>(
        "SELECT id, content, parent_id FROM posts where parent_id is ? ORDER BY created DESC",
    )
    .bind(post_id)
    .fetch_all(pool)
    .await?;
    Ok(posts)
}

pub async fn get_top_note(post_id: i64, pool: &SqlitePool) -> Result<Option<Post>> {
    let note = sqlx::query_as::<_, Post>(
        "select id, content, parent_id from posts where parent_id = ? order by random() limit 1",
    )
    .bind(post_id)
    .fetch_optional(pool)
    .await?;
    Ok(note)
}
