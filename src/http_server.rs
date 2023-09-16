use std::net::SocketAddr;

use crate::api;
use crate::http_static::static_handler;
use crate::pages::view_post::view_post;
use crate::pages::{self, create_post::create_post, vote::vote};
use anyhow::Result;
use axum::routing::post;
use axum::Extension;
use axum::{routing::get, Router};
use http::StatusCode;
use pages::frontpage::frontpage;
use pages::user::options::options;
use sqlx::SqlitePool;
use tower_cookies::CookieManagerLayer;
use tower_http::compression::CompressionLayer;
use tower_http::trace::TraceLayer;
use tracing::info;

pub async fn start_http_server(sqlite_pool: SqlitePool) -> Result<()> {
    let mut app = Router::new();

    app = app
        .route("/", get(frontpage))
        .route("/create_post", post(create_post))
        .route("/view_post/:post_id", get(view_post))
        .route("/vote", post(vote))
        .route("/options", get(options));

    let apiv0 = Router::new()
        .route("/user/create", post(api::create_user))
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
