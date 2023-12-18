use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
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

    // println!("{:#?}", rows);

    (StatusCode::OK, Json(rows))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestPost {
    title: String,
    author_id: i32,
    body: String,
    post_type: String,
    parent_id: Option<i32>,
}

//发布帖子
pub async fn push_post_handler(
    State(pool): State<Arc<sqlx::MySqlPool>>,
    Json(request_body): Json<RequestPost>,
) -> impl IntoResponse {
    // Access request body data
    let post_id = sqlx::query_scalar("SELECT MAX(PostID) FROM Posts")
        .fetch_one(&*pool)
        .await
        .map_or(1, |max_id: Option<i32>| max_id.unwrap_or(0) + 1);
    let title = request_body.title;
    let body = request_body.body;
    let author_id = request_body.author_id;
    let published_at = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let status = "published";
    let post_type = request_body.post_type;
    let parent_id = request_body.parent_id;

    sqlx::query("insert into Posts(PostID,Title,Content,AuthorID,PublishedAt,Status,Type,ParentPostID) values(?,?,?,?,?,?,?,?)")
        .bind(post_id).bind(title).bind(body).bind(author_id).bind(published_at).bind(status).bind(post_type).bind(parent_id)
        .execute(&*pool).await.unwrap();

    // Perform account registration logic here
    // Respond with a success message (modify accordingly)
    (StatusCode::OK, "success")
}

pub async fn search_author_post_handler(
    State(pool): State<Arc<sqlx::MySqlPool>>,
    Path(Authorname): Path<String>,
) -> impl IntoResponse {
    let author_name = Authorname;

    let rows = sqlx::query(
        "select PostID,Title,Username,DATE_FORMAT(PublishedAt, '%Y-%m-%d %H:%i:%s') as PublishedAt,
                         Content,Type,ParentPostID 
                         from Posts,Users
                         where Posts.AuthorID=Users.UserID and Users.Username=?",
    )
    .bind(author_name)
    .map(|row: sqlx::mysql::MySqlRow| Post {
        id: row.get(0),
        title: row.get(1),
        author: row.get(2),
        timestamp: row.get(3),
        body: row.get(4),
        post_type: row.get(5),
        parent_id: row.get(6),
    })
    .fetch_all(&*pool)
    .await
    .unwrap();

    // println!("{:#?}", rows);

    (StatusCode::OK, Json(rows))
}
