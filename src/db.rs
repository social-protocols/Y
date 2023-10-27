use anyhow::{anyhow, Result};
use common::structs::{Direction, Post};
use sqlx::SqlitePool;

// TODO: transactional
pub async fn create_post(
    tag: &str,
    parent_id: Option<i64>,
    content: &str,
    author_id: i64,
    pool: &SqlitePool,
) -> Result<i64> {
    let created_post_id = sqlx::query_scalar::<_, i64>(
        r#"
            insert into posts (content, parent_id, author_id)
            values (?, ?, ?)
            returning id
        "#,
    )
    .bind(content)
    .bind(parent_id)
    .bind(author_id)
    .fetch_one(pool)
    .await?;

    vote(author_id, tag, created_post_id, None, Direction::Up, pool).await?;

    Ok(created_post_id)
}

pub async fn get_post(post_id: i64, pool: &SqlitePool) -> Result<Option<Post>> {
    let post = sqlx::query_as::<_, Post>(
        r#"
            select
                  id
                , content
                , parent_id
                , author_id
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
    tag: &str,
    post_id: i64,
    note_id: Option<i64>,
    direction: Direction,
    pool: &SqlitePool,
) -> Result<()> {
    let direction_i32 = direction as i32;

    let tag_id = get_or_insert_tag_id(tag, pool).await?;

    sqlx::query(
        r#"
            with parameters as (
                select
                    ? as user_id,
                    ? as tag_id,
                    ? as post_id,
                    ? as note_id,
                    ? as direction
            )
            , duplicates as (
                select
                      parameters.user_id
                    , parameters.tag_id
                    , parameters.post_id
                    , parameters.direction == ifnull(current_vote.direction, 0) as duplicate
                from parameters
                left join current_vote using (user_id, tag_id, post_id)
            )
            insert into vote_history (
                  user_id
                , tag_id
                , post_id
                , direction
            )
            select
                  parameters.user_id
                , parameters.tag_id
                , parameters.post_id
                , parameters.direction
            from parameters
            join duplicates
            where not duplicate
        "#,
    )
    .bind(user_id)
    .bind(tag_id)
    .bind(post_id)
    .bind(note_id)
    .bind(direction_i32)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_or_insert_tag_id(tag: &str, pool: &SqlitePool) -> Result<i64> {
    let tag_id = sqlx::query_scalar::<_, i64>(
        r#"
            insert or ignore into tags (tag) values (?)
            returning id
        "#,
    )
    .bind(tag)
    .fetch_optional(pool)
    .await?;

    Ok(match tag_id {
        None => get_tag_id(tag, pool).await?.unwrap(),
        Some(tag_id) => tag_id,
    })
}

async fn get_tag_id(tag: &str, pool: &SqlitePool) -> Result<Option<i64>> {
    let tag_id = sqlx::query_scalar::<_, i64>(
        r#"
            select id
            from tags
            where tag = ?
        "#,
    )
    .bind(tag)
    .fetch_optional(pool)
    .await?;

    Ok(tag_id)
}

pub async fn list_top_level_posts(tag: &str, pool: &SqlitePool) -> Result<Vec<Post>> {
    let tag_id = get_tag_id(tag, pool).await?;
    let result = match tag_id {
        Some(tag_id) => {
            sqlx::query_as::<_, Post>(
                r#"
                    select
                        id
                        , content
                        , parent_id
                        , author_id
                    from posts
                    join current_tally ct
                    on posts.id = ct.post_id
                    and ct.tag_id = ?
                    where posts.parent_id is null
                    order by ct.upvotes * (1 + log(ct.upvotes / ct.votes)) desc
                "#,
            )
            .bind(tag_id)
            .fetch_all(pool)
            .await?
        }
        None => vec![],
    };
    Ok(result)
}

pub async fn get_replies(tag: &str, post_id: i64, pool: &SqlitePool) -> Result<Vec<Post>> {
    // TODO: sort replies by score for tag
    let tag_id = get_tag_id(tag, pool).await?;

    let posts = sqlx::query_as::<_, Post>(
        r#"
            select
                  id
                , content
                , parent_id
            from posts
            join current_tally ct
            on posts.id = ct.post_id
            and ct.tag_id = ?
            where parent_id is ?
            order by ct.upvotes * (1 + log(ct.upvotes / ct.votes)) desc
        "#,
    )
    .bind(tag_id)
    .bind(post_id)
    .fetch_all(pool)
    .await?;
    Ok(posts)
}

pub async fn get_top_note(tag: &str, post_id: i64, pool: &SqlitePool) -> Result<Option<Post>> {
    Ok(
        match crate::probabilities::find_top_note(post_id, pool).await? {
            None => None,
            Some((note_id, _, _)) => get_post(note_id, pool).await?,
        },
    )
}

fn normalize_tag(tag: &str) -> String {
    tag.to_lowercase()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect()
}

pub async fn get_top_level_posts_with_tag(tag: &str, pool: &SqlitePool) -> Result<Vec<Post>> {
    let tag_id = get_tag_id(tag, pool).await?;
    let result = match tag_id {
        Some(tag_id) => {
            sqlx::query_as::<_, Post>(
                r#"
                    select *
                    from posts
                    join current_tally ct
                    on posts.id = ct.post_id
                    and ct.tag_id = ?
                    where tags.tag = ?
                    and parent_id is null
                "#,
            )
            .bind(tag_id)
            .fetch_all(pool)
            .await?
        }
        None => vec![],
    };
    Ok(result)
}

pub async fn get_top_5_tags(pool: &SqlitePool) -> Result<Vec<String>> {
    let result = sqlx::query_scalar::<_, String>(
        r#"
            select tag
            from current_tally
            group by tag_id
            order by count(*) desc
            limit 5
        "#,
    )
    .fetch_all(pool)
    .await?;
    Ok(result)
}
