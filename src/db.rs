//! Database access via sqlx

use anyhow::Result;
use sqlx::SqlitePool;

use crate::structs::Post;

pub async fn create_post(content: &str, pool: &SqlitePool) -> Result<i64> {
    let created_post_id =
        sqlx::query_scalar::<_, i64>("INSERT INTO posts (content) VALUES (?) RETURNING id")
            .bind(content)
            .fetch_one(pool)
            .await?;
    Ok(created_post_id)
}

pub async fn list_posts(pool: &SqlitePool) -> Result<Vec<Post>> {
    let posts = sqlx::query_as::<_, Post>("SELECT id, content FROM posts ORDER BY created DESC")
        .fetch_all(pool)
        .await?;
    Ok(posts)
}
