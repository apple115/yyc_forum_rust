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
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Debug, Deserialize)]
pub struct Users {
    id: i32,
    username: String,
    password: String,
    email: String,
    register_time: String,
    user_type: String,
}

#[derive(Debug, Deserialize)]
pub struct RequestRegister {
    username: String,
    password: String,
    email: String,
}

#[derive(Debug, Deserialize)]
pub struct RequestLogin {
    username: String,
    password: String,
}

#[derive(Debug, Deserialize)]
pub struct login_data {
    username: String,
    password: String,
    user_role: String,
}

pub async fn register_handler(
    State(pool): State<Arc<sqlx::MySqlPool>>,
    Json(request_body): Json<RequestRegister>,
) -> impl IntoResponse {
    // Access request body data
    // userID 添加自增 找到mysql的最大id 然后+1
    //
    let user_id = sqlx::query_scalar("SELECT MAX(UserID) FROM Users")
        .fetch_one(&*pool)
        .await
        .map_or(1, |max_id: Option<i32>| max_id.unwrap_or(0) + 1);
    let username = request_body.username;
    let password = request_body.password;
    let email = request_body.email;
    let register_time = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let user_type = "User";

    //插入数据
    let row = sqlx::query(
        "INSERT INTO Users (UserID, Username, PasswordHash, Email, RegistrationTime, UserRole) VALUES (?,?,?,?,?,?)"
        ).bind(user_id).bind(username).bind(password).bind(email).bind(register_time).bind(user_type)
        .execute(&*pool)
        .await.expect("Failed to insert data");

    (StatusCode::OK, "Registration successful")
}

#[derive(Debug, Serialize)]
pub struct Returndata {
    useid: Option<i32>,
    username: Option<String>,
    userrole: Option<String>,
}

pub async fn login_handler(
    State(pool): State<Arc<sqlx::MySqlPool>>,
    Json(request_body): Json<RequestLogin>,
) -> impl IntoResponse {
    // Perform login logic here
    // For demonstration, just print the received data
    let users = sqlx::query("select Username,PasswordHash,UserRole from Users")
        .map(|row: sqlx::mysql::MySqlRow| login_data {
            username: row.get(0),
            password: row.get(1),
            user_role: row.get(2),
        })
        .fetch_all(&*pool)
        .await
        .unwrap();

    let username = request_body.username;
    let password = request_body.password;
    //如果用户名密码处于login data中，则返回成功
    let mut returndata = Vec::new();
    for user in users.iter() {
        if user.username == username && user.password == password {
            returndata = sqlx::query(
                "SELECT UserID,Username,UserRole FROM Users WHERE Username=? and PasswordHash=?",
            )
            .bind(username)
            .bind(password)
            .map(|map: sqlx::mysql::MySqlRow| Returndata {
                useid: map.get(0),
                username: map.get(1),
                userrole: map.get(2),
            })
            .fetch_all(&*pool)
            .await
            .unwrap();

            return (StatusCode::OK, Json(returndata));
        }
    }
    return (StatusCode::OK, Json(returndata));
}
