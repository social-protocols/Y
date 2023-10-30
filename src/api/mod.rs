use anyhow::{anyhow, Result};

use crate::constants::GLOBAL_TAG;

use axum::{
    extract::{self, Path},
    headers::{authorization::Bearer, Authorization},
    Extension, Json, TypedHeader,
};

use common::{
    auth,
    structs_api::{ApiCreatePost, ApiFrontpage, ApiPost, ApiPostPage, ApiVote},
};
use sqlx::SqlitePool;

use crate::{db, error::AppError};

pub async fn create_user(Extension(pool): Extension<SqlitePool>) -> Result<String, AppError> {
    let user = auth::create_user(&pool).await?;
    Ok(user.secret)
}


pub async fn frontpage(
    Extension(pool): Extension<SqlitePool>,
) -> Result<Json<ApiFrontpage>, AppError> {
    let tag = GLOBAL_TAG;
    let posts = db::get_posts_for_tag(tag, &pool).await?;
    Ok(Json(ApiFrontpage {
        posts: posts
            .iter()
            .map(|post| ApiPost {
                id: post.id,
                content: post.content.clone(),
            })
            .collect(),
    }))
}

pub async fn view_post(
    Path(post_id): Path<i64>,
    Extension(pool): Extension<SqlitePool>,
) -> Result<Json<Option<ApiPostPage>>, AppError> {
    let post = db::get_post(post_id, &pool).await?;
    let tag = GLOBAL_TAG;
    Ok(Json(match post {
        Some(post) => {
            let parent_context = db::get_transitive_parents(&post, &pool).await?;
            let top_note = db::get_top_note(tag, post_id, &pool).await?;
            let replies = db::get_replies(tag, post_id, &pool).await?;
            Some(ApiPostPage {
                parent_context: parent_context
                    .iter()
                    .rev()
                    .map(|post| ApiPost {
                        id: post.id,
                        content: post.content.clone(),
                    })
                    .collect(),
                post: ApiPost {
                    id: post.id,
                    content: post.content.clone(),
                },
                note: top_note.map(|note| ApiPost {
                    id: note.id,
                    content: note.content.clone(),
                }),
                replies: replies
                    .iter()
                    .map(|post| ApiPost {
                        id: post.id,
                        content: post.content.clone(),
                    })
                    .collect(),
            })
        }
        None => None,
    }))
}

// curl -v http://127.0.0.1:8000/api/v0/vote -d '{"post_id": 2, "note_id": 17, "direction": "Down"}' -H "Authorization: Bearer xxxxxxxxx" -H "Content-Type: application/json"
pub async fn vote(
    Extension(pool): Extension<SqlitePool>,
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    extract::Json(payload): extract::Json<ApiVote>,
) -> Result<(), AppError> {
    // TODO: is it possible to get user from baerer token using axum middleware?
    let secret = bearer.token();
    let user = auth::user_from_secret(secret, &pool)
        .await?
        .ok_or(anyhow!("Unauthorized"))?; // TODO: return proper HTTP header, by sending a

    // TODO: better http status code if post/note doesn't exist
    db::vote(
        user.id,
        payload.tag.as_str(),
        payload.post_id,
        payload.note_id,
        payload.direction.to_direction(),
        &pool,
    )
    .await?;

    Ok(())
}

pub async fn create_post(
    Extension(pool): Extension<SqlitePool>,
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    extract::Json(payload): extract::Json<ApiCreatePost>,
) -> Result<(), AppError> {
    // TODO: is it possible to get user from baerer token using axum middleware?
    let secret = bearer.token();
    let user = auth::user_from_secret(secret, &pool)
        .await?
        .ok_or(anyhow!("Unauthorized"))?; // TODO: return proper HTTP header, by sending a

    let tag = GLOBAL_TAG;

    // TODO: better http status code if post/note doesn't exist
    db::create_post(tag, payload.parent_id, &payload.content, user.id, &pool).await?;

    Ok(())
}
