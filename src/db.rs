//! Database access via sqlx

use anyhow::Result;
use sqlx::SqlitePool;

use crate::structs::Post;

use crate::structs::Direction;

pub async fn create_post(content: &str, pool: &SqlitePool) -> Result<i64> {
    let created_post_id =
        sqlx::query_scalar::<_, i64>("INSERT INTO posts (content) VALUES (?) RETURNING id")
            .bind(content)
            .fetch_one(pool)
            .await?;
    Ok(created_post_id)
}

pub async fn vote(
    user_id: i64,
    post_id: i64,
    note_id: Option<i64>,
    direction: Direction,
    pool: &SqlitePool,
) -> Result<()> {
    let direction_i32 = direction as i32;

    sqlx::query("INSERT INTO vote_history (user_id, post_id, note_id, direction) VALUES (?, ?,?,?)")
        .bind(user_id)
        .bind(post_id)
        .bind(note_id)
        .bind(direction_i32)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn list_posts(pool: &SqlitePool) -> Result<Vec<Post>> {
    let posts = sqlx::query_as::<_, Post>("SELECT id, content FROM posts ORDER BY created DESC")
        .fetch_all(pool)
        .await?;
    Ok(posts)
}
