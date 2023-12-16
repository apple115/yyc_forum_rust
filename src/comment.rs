use axum::{
    extract::Path,
    http::{HeaderValue, Request, Response, StatusCode},
    response::{Html, IntoResponse},
    routing::{get, post},
    Json, Router,
};
use http::{
    header::{self, CONTENT_TYPE},
    Method,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub struct Comment {
    pub id: u32,
    pub user_id: u32,
    pub post_id: u32,
    pub content: String,
}
