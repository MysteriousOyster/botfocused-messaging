/*
 * Copyright (C) 2026 Leif Barton
 * Licensed under the Open Software License 3.0
 */

use std::sync::Arc;

use argon2::{Argon2, PasswordHasher, password_hash::{SaltString, rand_core::OsRng}};
use axum::{
    Json, Router, extract::State, http::{HeaderValue, Method, StatusCode, header}, routing::{get, post}
};
use axum_extra::extract::cookie::Key;
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;
use tower_service::Service;
use worker::*;

#[derive(Clone, Debug)]
pub struct GlobalState {
    db: Arc<D1Database>,
    cookie_key: Key,
}

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    let database = env.d1("DB").unwrap();

    let mut router = Router::new()
        .route("/", get(root).post(root))
        .route("/git", get(git_info))
        .route("/users", get(get_users).post(get_users))
        .route("/newuser", post(new_user))
        .with_state(GlobalState {
            db: Arc::new(database),
            // Enforce a COOKIE_KEY if not in development environment
            cookie_key: env.var("COOKIE_KEY").map(|o| {
                if env.var("ENV").unwrap().to_string() == "dev" {
                    Key::from(o.to_string().as_bytes())
                } else {
                    panic!("COOKIE_KEY env var must be present in prod!")
                }
            }).unwrap_or_else(|_| Key::generate())
        })
        .layer(
            CorsLayer::new()
                .allow_origin("https://bot.leifbarton.dev".parse::<HeaderValue>().unwrap())
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::OPTIONS,
                    Method::PUT,
                    Method::PATCH,
                    Method::DELETE,
                    Method::HEAD,
                ])
                .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
                .allow_credentials(true),
        );

    Ok(router.call(req).await?)
}

#[derive(Deserialize, Serialize)]
struct UserDataRow {
    id: String,
    username: String,
    password: String,
}

#[worker::send]
pub async fn get_users(State(state): State<GlobalState>) -> Json<Vec<UserDataRow>> {
    let db = &*state.db;
    let res: Vec<UserDataRow> = db
        .prepare("SELECT * FROM user")
        .all()
        .await
        .unwrap()
        .results()
        .unwrap();
    Json(res)
}

#[derive(Deserialize)]
struct NewUserRequest {
    username: String,
    password: String,
}

#[worker::send]
pub async fn new_user(
    State(state): State<GlobalState>,
    Json(payload): Json<NewUserRequest>,
) -> Result<(), (StatusCode, String)> {
    let argon2 = Argon2::default();
    
    let salt = SaltString::generate(&mut OsRng);

    let password = argon2.hash_password(payload.password.as_bytes(), &salt).unwrap();
    let db = &*state.db;
    
    let thing = db.prepare("INSERT INTO user (username, password) VALUES (?1, ?2)")
        .bind(&[payload.username.into(), format!("{password}").into()])
        .unwrap();
    dbg!(&thing);
    thing
        .run()
        .await
        .map_err(|e| (StatusCode::UNPROCESSABLE_ENTITY, format!("{e}")))
        .map(|_| ())
}

pub async fn root() -> &'static str {
    "hello!"
}

pub async fn git_info() -> &'static str {
    concat!(
        "SHA: ",
        env!("VERGEN_GIT_SHA"),
        "\nBranch: ",
        env!("VERGEN_GIT_BRANCH"),
        "\nCommit Message: ",
        env!("VERGEN_GIT_COMMIT_MESSAGE")
    )
}
