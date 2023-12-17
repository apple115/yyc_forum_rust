use axum::{
    extract::{Path, State},
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
use sqlx::{mysql::MySqlPoolOptions, FromRow, MySqlPool, Row};

use std::collections::HashMap;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Serialize, Deserialize)]
pub struct Comment {
    pub id: i32,
    pub username: String,
    pub content: String,
    pub publishedat: String,
}

#[derive(Serialize, Deserialize)]
pub struct RequestComment {
    content: String,
    author_id: u32,
    post_id: u32,
}

pub async fn push_comment_handler(
    State(pool): State<Arc<sqlx::MySqlPool>>,
    Json(request_body): Json<RequestComment>,
) -> impl IntoResponse {
    // Access request body data
    let comment_id = sqlx::query_scalar("SELECT MAX(CommentID) FROM Comments")
        .fetch_one(&*pool)
        .await
        .map_or(1, |max_id: Option<i32>| max_id.unwrap_or(0) + 1);
    let author_id = request_body.author_id;
    let post_id = request_body.post_id;
    let content = request_body.content;
    let publishedat = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    sqlx::query(
        "INSERT INTO Comments(CommentID,Content,AuthorID,PostID,PublishedAt) VALUES(?,?,?,?,?)",
    )
    .bind(comment_id)
    .bind(content)
    .bind(author_id)
    .bind(post_id)
    .bind(publishedat)
    .execute(&*pool)
    .await
    .unwrap();

    // Perform account registration logic here
    // Respond with a success message (modify accordingly)
    (StatusCode::OK, "pushcommnet successful")
}

pub async fn get_comments_data_handler(
    State(pool): State<Arc<sqlx::MySqlPool>>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    // TODO
    let post_id = id;
    let comments=sqlx::query("
select DISTINCT c.CommentID ,u.Username,DATE_FORMAT(c.PublishedAt, '%Y-%m-%d %H:%i:%s') as PublishedAt ,c.Content
from Posts p,Users u,Comments c  
where p.AuthorID =u.UserID and c.AuthorID =u.UserID and c.PostID =?"
)
        .bind(post_id)
        .map(
            |row: sqlx::mysql::MySqlRow| {
                Comment {
                    id: row.get(0),
                    username: row.get(1),
                    content: row.get(3),
                    publishedat: row.get(2),
                }}).fetch_all(&*pool).await.unwrap();

    // Respond with a success message (modify accordingly)
    (StatusCode::OK, Json(comments))
}
