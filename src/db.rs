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

pub async fn get_post(post_id: i64, pool: &SqlitePool) -> Result<Option<Post>> {
    let post = sqlx::query_as::<_, Post>("select id, content, parent_id from posts where id = ?")
        .bind(post_id)
        .fetch_optional(pool)
        .await?;

    Ok(post)
}

pub async fn get_transitive_parents(post_id: i64, pool: &SqlitePool) -> Result<Vec<Post>> {
    // loop until a post without parent_id is found
    let mut post_id = post_id;
    let mut parents: Vec<Post> = vec![];
    while let Some(post) = get_post(post_id, pool).await? {
        match post.parent_id {
            None => break,
            Some(parent_id) => {
                parents.push(post);
                post_id = parent_id;
            }
        }
    }
    Ok(parents)
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

pub async fn get_replies(post_id: i64, pool: &SqlitePool) -> Result<Vec<Post>> {
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
        // for now, order consistently until we have a scoring formula.
        "select id, content, parent_id from posts where parent_id = ? order by created desc limit 1",
    )
    .bind(post_id)
    .fetch_optional(pool)
    .await?;
    Ok(note)
}

pub async fn get_current_vote(post_id: i64, user_id: i64, pool: &SqlitePool) -> Result<Direction> {
    let vote = sqlx::query_scalar::<_, i32>(
        "select direction from current_vote where post_id = ? and user_id = ?",
    )
    .bind(post_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    Direction::from(vote.unwrap_or(0))
}
