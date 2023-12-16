use axum::{
    extract::Path,
    http::{HeaderValue, Request, Response, StatusCode},
    response::{Html, IntoResponse},
    routing::{get, post},
    Extension, Json, Router,
};
use http::{
    header::{self, CONTENT_TYPE},
    Method,
};
use serde::{Deserialize, Serialize};
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
mod posts;
use posts::{get_post_by_id_handler, get_posts_data_handler, push_post_handler};
mod users;
use sqlx::mysql::MySqlPoolOptions;
use std::sync::Arc;
use users::{login_handler, register_handler};

async fn init_pool() -> sqlx::MySqlPool {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://root:Yyc13714729559!@localhost:3306/forum")
        .await
        .expect("Failed to create pool.");
    pool
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_global_404_handler=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let pool = Arc::new(init_pool().await);

    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .route("/register", post(register_handler))
        .route("/login", post(login_handler))
        .route("/pushpost", post(push_post_handler))
        .route("/posts", get(get_posts_data_handler))
        .route("/posts/:id", get(get_post_by_id_handler))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([Method::GET, Method::POST])
                .allow_headers([CONTENT_TYPE]),
        )
        .with_state(pool);

    // add a fallback service for handling routes to unknown paths
    let app = app.fallback(handler_404);

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
