use std::net::SocketAddr;

use crate::api;
use crate::http_static::static_handler;
use crate::pages::{
    self, communities::community_frontpage, create_post::create_post, positions::positions,
    tags::add_tag, view_post::view_post, vote::vote,
};
use anyhow::Result;
use axum::{
    routing::{get, post},
    Extension, Router,
};
use http::StatusCode;
use pages::{frontpage::frontpage, user::options::options};
use sqlx::SqlitePool;
use tower_cookies::CookieManagerLayer;
use tower_http::{compression::CompressionLayer, trace::TraceLayer};
use tracing::info;

pub async fn start_http_server(sqlite_pool: SqlitePool) -> Result<()> {
    let mut app = Router::new();

    app = app
        .route("/", get(frontpage))
        .route("/s/:tag", get(community_frontpage))
        .route("/create_post", post(create_post))
        .route("/view_post/:post_id", get(view_post))
        .route("/vote", post(vote))
        .route("/positions", get(positions))
        .route("/options", get(options))
        .route("/add_tag", post(add_tag));

    let apiv0 = Router::new()
        .route("/user/create", post(api::create_user))
        .route("/frontpage", get(api::frontpage))
        .route("/view_post/:post_id", get(api::view_post))
        .route("/vote", post(api::vote))
        .layer(Extension(sqlite_pool.clone()));

    app = app
        .route("/healthy", get(handler_healthy))
        .route("/*file", get(static_handler))
        .layer(TraceLayer::new_for_http())
        .layer(Extension(sqlite_pool.to_owned()))
        .layer(CookieManagerLayer::new())
        .layer(CompressionLayer::new())
        .fallback_service(get(not_found));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    info!("Http server listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.nest("/api/v0", apiv0).into_make_service())
        .await?;

    Ok(())
}

async fn handler_healthy() -> StatusCode {
    StatusCode::OK
}

async fn not_found() -> StatusCode {
    StatusCode::NOT_FOUND
}
