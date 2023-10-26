use anyhow::{anyhow, Result};
use common::structs::{Direction, Post};
use sqlx::SqlitePool;

pub async fn create_post(content: &str, parent_id: Option<i64>, pool: &SqlitePool) -> Result<i64> {
    let created_post_id = sqlx::query_scalar::<_, i64>(
        r#"
            INSERT INTO posts (content, parent_id)
            VALUES (?, ?)
            RETURNING id
        "#,
    )
    .bind(content)
    .bind(parent_id)
    .fetch_one(pool)
    .await?;
    Ok(created_post_id)
}

pub async fn get_post(post_id: i64, pool: &SqlitePool) -> Result<Option<Post>> {
    let post = sqlx::query_as::<_, Post>(
        r#"
            SELECT
                  id
                , content
                , parent_id
            FROM posts
            WHERE id = ?
        "#,
    )
    .bind(post_id)
    .fetch_optional(pool)
    .await?;

    Ok(post)
}

pub async fn get_transitive_parents(post: &Post, pool: &SqlitePool) -> Result<Vec<Post>> {
    let mut parents: Vec<Post> = vec![];
    let mut p = post.clone();

    // loop until a post without parent_id is found
    while let Some(parent_id) = p.parent_id {
        match get_post(parent_id, pool).await? {
            None => return Err(anyhow!("Couldn't find post with id: {}", parent_id)),
            Some(parent_post) => {
                parents.push(parent_post.clone());
                p = parent_post;
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
        r#"
            WITH parameters AS (
                SELECT
                    ? AS user_id,
                    ? AS post_id,
                    ? AS note_id,
                    ? AS direction
            )
            , duplicates AS (
                SELECT
                      parameters.user_id
                    , parameters.post_id
                    , parameters.direction == IFNULL(current_vote.direction, 0) AS duplicate
                FROM parameters
                LEFT JOIN current_vote USING (user_id, post_id)
            )
            INSERT INTO vote_history (
                  user_id
                , post_id
                , direction
            )
            SELECT
                  parameters.user_id
                , parameters.post_id
                , parameters.direction
            FROM parameters
            JOIN duplicates
            WHERE NOT duplicate
        "#,
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
        r#"
            SELECT
                  id
                , content
                , parent_id
            FROM posts
            WHERE parent_id IS NULL
            ORDER BY created DESC
        "#,
    )
    .fetch_all(pool)
    .await?;
    Ok(posts)
}

pub async fn get_replies(post_id: i64, pool: &SqlitePool) -> Result<Vec<Post>> {
    let posts = sqlx::query_as::<_, Post>(
        r#"
            SELECT
                  id
                , content
                , parent_id
            FROM posts
            WHERE parent_id IS ?
            ORDER BY created DESC
        "#,
    )
    .bind(post_id)
    .fetch_all(pool)
    .await?;
    Ok(posts)
}

pub async fn get_top_note(post_id: i64, pool: &SqlitePool) -> Result<Option<Post>> {
    Ok(
        match crate::probabilities::find_top_note(post_id, pool).await? {
            None => None,
            Some((note_id, _, _)) => get_post(note_id, pool).await?,
        },
    )
}

pub async fn add_tag(post_id: i64, tag: &str, pool: &SqlitePool) -> Result<()> {
    let normalized_tag = normalize_tag(tag);
    sqlx::query(
        r#"
            INSERT OR IGNORE INTO tags (post_id, tag)
            VALUES (?, ?)
        "#,
    )
    .bind(post_id)
    .bind(normalized_tag)
    .execute(pool)
    .await?;
    Ok(())
}

fn normalize_tag(tag: &str) -> String {
    tag.to_lowercase()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect()
}

pub async fn get_top_level_posts_with_tag(tag: &str, pool: &SqlitePool) -> Result<Vec<Post>> {
    let result = sqlx::query_as::<_, Post>(
        r#"
            SELECT *
            FROM posts
            JOIN tags
            ON posts.id = tags.post_id
            WHERE tags.tag = ?
            AND parent_id IS NULL
        "#,
    )
    .bind(tag)
    .fetch_all(pool)
    .await?;
    Ok(result)
}

pub async fn get_top_5_tags(pool: &SqlitePool) -> Result<Vec<String>> {
    let result = sqlx::query_scalar::<_, String>(
        r#"
            SELECT tag
            FROM tags
            GROUP BY tag
            ORDER BY COUNT(*) DESC
            LIMIT 5
        "#,
    )
    .fetch_all(pool)
    .await?;
    Ok(result)
}
