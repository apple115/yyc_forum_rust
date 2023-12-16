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
    let username = request_body.username;
    let password = request_body.password;
    let email = request_body.email;

    // Perform account registration logic here

    // For demonstration, just print the received data
    println!(
        "Received registration request - Username: {}, Password: {},email: {}",
        username, password, email
    );

    // Respond with a success message (modify accordingly)
    (StatusCode::OK, "Registration successful")
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

    // println!("{:#?}", Users);

    let username = request_body.username;
    let password = request_body.password;
    //如果用户名密码处于login data中，则返回成功
    for user in users {
        if user.username == username && user.password == password {
            return (StatusCode::OK, "Login successful");
        }
        return (StatusCode::OK, "Login failed");
    }
    return (StatusCode::OK, "Login failed");
}
