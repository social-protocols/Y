use common::structs::{Direction, Post};

use anyhow::{anyhow, Result};

use sqlx::SqlitePool;

pub async fn create_post(content: &str, parent_id: Option<i64>, pool: &SqlitePool) -> Result<i64> {
    let created_post_id = sqlx::query_scalar::<_, i64>(
        r#"
            insert into posts (content, parent_id)
            values (?, ?)
            returning id
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
            select
                id
                , content
                , parent_id
            from posts
            where id = ?
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
            with parameters as (
                select 
                    ? as user_id,
                    ? as post_id,
                    ? as note_id,
                    ? as direction
            )
            , duplicates as (
                select
                    parameters.user_id
                    , parameters.post_id
                    , parameters.direction == ifnull(current_vote.direction,0) as duplicate
                from parameters 
                left join current_vote using (user_id, post_id)
            )
            insert into vote_history(
                user_id
                , post_id
                , direction
            ) 
            select 
                parameters.user_id
                , parameters.post_id
                , parameters.direction
            from parameters
            join duplicates
            where not duplicate
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
            select
                id
                , content
                , parent_id
            from posts
            where parent_id is null
            order by created desc
        "#,
    )
    .fetch_all(pool)
    .await?;
    Ok(posts)
}

pub async fn get_replies(post_id: i64, pool: &SqlitePool) -> Result<Vec<Post>> {
    let posts = sqlx::query_as::<_, Post>(
        r#"
            select
                id
                , content
                , parent_id
            from posts
            where parent_id is ?
            order by created desc
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

pub async fn add_hashtag(post_id: i64, hashtag: &str, pool: &SqlitePool) -> Result<()> {
    sqlx::query(
        r#"
            INSERT INTO hashtag (post_id, hashtag)
            VALUES (?, ?)
        "#,
    )
    .bind(post_id)
    .bind(hashtag)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_top_level_posts_with_hashtag(
    hashtag: &str,
    pool: &SqlitePool,
) -> Result<Vec<Post>> {
    let result = sqlx::query_as::<_, Post>(
        r#"
            SELECT *
            FROM posts
            JOIN hashtag
            ON posts.id = hashtag.post_id
            WHERE hashtag.hashtag = ?
            AND parent_id IS NULL
        "#,
    )
    .bind(hashtag)
    .fetch_all(pool)
    .await?;
    Ok(result)
}

pub async fn get_top_5_hashtags(pool: &SqlitePool) -> Result<Vec<String>> {
    let result = sqlx::query_scalar::<_, String>(
        r#"
            SELECT hashtag
            FROM hashtag
            GROUP BY hashtag
            ORDER BY COUNT(*) DESC
            LIMIT 5
        "#,
    )
    .fetch_all(pool)
    .await?;
    Ok(result)
}
