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

use chrono::prelude::*;
use std::sync::Arc;

#[derive(Debug, Serialize, FromRow)]
pub struct Post {
    id: i32,
    title: String,
    author: String,
    timestamp: String,
    body: String,
    post_type: String,
    parent_id: Option<i32>,
}

pub async fn get_posts_data_handler(State(pool): State<Arc<sqlx::MySqlPool>>) -> impl IntoResponse {
    let posts=sqlx::query("select PostID,Title,Username,DATE_FORMAT(PublishedAt, '%Y-%m-%d %H:%i:%s') as PublishedAt,Content,Type,ParentPostID from Posts,Users where Posts.AuthorID=Users.UserID")
        .map(
            |row: sqlx::mysql::MySqlRow| {
                Post {
                    id: row.get(0),
                    title: row.get(1),
                    author:row.get(2),
                    timestamp: row.get(3),
                    body: row.get(4),
                    post_type: row.get(5),
                    parent_id: row.get(6),
                }}).fetch_all(&*pool).await.unwrap();

    println!("{:#?}", posts);

    // Respond with a success message (modify accordingly)
    (StatusCode::OK, Json(posts))
}

pub async fn get_post_by_id_handler(
    State(pool): State<Arc<sqlx::MySqlPool>>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let post_id = id;

    // Fetch the post with the specified ID (you need to implement the logic)
    let rows=sqlx::query("select PostID,Title,Username,DATE_FORMAT(PublishedAt, '%Y-%m-%d %H:%i:%s') as PublishedAt,Content,Type,ParentPostID from Posts,Users where Posts.AuthorID=Users.UserID and PostID=?")
        .bind(post_id)
        .map(
            |row: sqlx::mysql::MySqlRow| {
                Post {
                    id: row.get(0),
                    title: row.get(1),
                    author:row.get(2),
                    timestamp: row.get(3),
                    body: row.get(4),
                    post_type: row.get(5),
                    parent_id: row.get(6),
                }}).fetch_all(&*pool).await.unwrap();

    println!("{:#?}", rows);

    (StatusCode::OK, Json(rows))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestPost {
    title: String,
    author: String,
    body: String,
    parent_id: i32,
}

//发布帖子
pub async fn push_post_handler(Json(request_body): Json<RequestPost>) -> impl IntoResponse {
    // Access request body data
    let title = request_body.title;
    let author = request_body.author;
    // let timestamp =
    let body = request_body.body;
    // let post_type = request_body.post_type;
    let parent_id = request_body.parent_id;

    // Perform account registration logic here
    // For demonstration, just print the received data
    println!(
        "Received registration request - title: {}, author: {}, body: {}, parent_id: {}",
        title, author, body, parent_id
    );
    // Respond with a success message (modify accordingly)
    (StatusCode::OK, "Registration successful")
}
