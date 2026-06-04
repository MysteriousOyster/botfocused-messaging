/*
 * Copyright (C) 2026 Leif Barton
 * Licensed under the Open Software License 3.0
 */

use std::sync::Arc;

#[derive(Clone, Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
enum PermissionLevel {
    YetToVerify = 20,
    Verified,
    Helper,
    Admin,
}

use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier, password_hash::{self, SaltString, rand_core::OsRng}
};
use axum::{
    extract::{FromRef, FromRequestParts, State},
    http::{header, HeaderValue, Method, StatusCode},
    routing::{get, post},
    Json, RequestPartsExt, Router,
};
use axum_extra::extract::{
    cookie::{Cookie, Key, SameSite},
    PrivateCookieJar,
};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use tower_http::cors::CorsLayer;
use tower_service::Service;
use worker::*;

#[derive(Clone, Debug)]
pub struct AppState {
    db: Arc<D1Database>,
    cookie_key: Key,
}

impl FromRef<AppState> for Key {
    fn from_ref(state: &AppState) -> Self {
        state.cookie_key.clone()
    }
}

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    let database = env.d1("DB").unwrap();

    let allowed_origin = if env
        .var("ENVENV")
        .map(|v| v.to_string() == "dev")
        .unwrap_or(false)
    {
        "http://localhost:4321".parse::<HeaderValue>().unwrap()
    } else {
        "https://bot.leifbarton.dev".parse::<HeaderValue>().unwrap()
    };

    let mut router = Router::new()
        .route("/", get(root).post(root))
        .route("/git", get(git_info))
        .route("/users", get(get_users).post(get_users))
        .route("/newuser", post(new_user))
        .route("/meuser", get(me_user))
        .route("/amiin", get(am_i_in))
        .route("/logout", post(log_out))
        .route("/login", post(log_in))
        .with_state(AppState {
            db: Arc::new(database),
            // Enforce a COOKIE_KEY if not in development environment
            cookie_key: env
                .var("COOKIE_KEY")
                .map(|o| Key::from(o.to_string().as_bytes()))
                .expect("COOKIE_KEY environment variable must be present"),
        })
        .layer(
            CorsLayer::new()
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::OPTIONS,
                    Method::PUT,
                    Method::DELETE,
                ])
                .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION, header::COOKIE])
                .allow_origin(allowed_origin)
                .allow_credentials(true),
        );

    Ok(router.call(req).await?)
}

#[derive(Deserialize, Serialize)]
struct UserDataRow {
    id: String,
    username: String,
    password: String,
    permission: PermissionLevel,
}

#[worker::send]
pub async fn get_users(State(state): State<AppState>) -> Json<Vec<UserDataRow>> {
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

#[derive(Serialize, Deserialize)]
struct SessionCookie {
    id: String,
    permission: PermissionLevel,
}

#[derive(Clone, Debug, Serialize)]
pub struct UserSession {
    id: String,
    permission: PermissionLevel,
}

impl From<SessionCookie> for UserSession {
    fn from(value: SessionCookie) -> Self {
        Self {
            id: value.id,
            permission: value.permission,
        }
    }
}

impl<S> FromRequestParts<S> for UserSession
where
    S: Send + Sync,
    Key: FromRef<S>,
{
    type Rejection = (StatusCode, String);
    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let jar = parts
            .extract_with_state::<PrivateCookieJar, S>(state)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Cookie error: {e}"),
                )
            })?;
        console_log!("{jar:?}");
        let session_cookie = jar
            .get("session")
            .ok_or((StatusCode::UNAUTHORIZED, "Please log in first.".into()))?;
        let session_cookie: SessionCookie =
            serde_json::from_str(session_cookie.value()).map_err(|_| {
                (
                    StatusCode::UNAUTHORIZED,
                    "Malformed cookie. Please log in first.".into(),
                )
            })?;
        Ok(session_cookie.into())
    }
}

fn set_session(
    jar: PrivateCookieJar,
    cookie: SessionCookie,
) -> Result<PrivateCookieJar, serde_json::Error> {
    let mut cookie = Cookie::new("session", serde_json::to_string(&cookie)?);
    // Use Lax for localhost development (different ports on same host)
    // In production with Secure flag, use None
    cookie.set_same_site(SameSite::Lax);
    cookie.set_http_only(true);
    Ok(jar.add(cookie))
}
fn remove_session(jar: PrivateCookieJar) -> PrivateCookieJar {
    jar.remove("session")
}
/*
fn get_session(jar: PrivateCookieJar) -> Result<SessionCookie, serde_json::Error> {
    jar.get("session")
        .ok_or(serde_json::Error::missing_field("cookie"))
        .map(|c| serde_json::from_str(c.value()))
        .flatten()
}
 */

#[derive(Deserialize, Clone, Debug)]
struct InsertedUser {
    id: String,
    permission: PermissionLevel,
}

#[derive(Serialize, Clone, Debug)]
struct User {
    id: String,
    username: String,
    permission: PermissionLevel,
}

impl From<UserDataRow> for User {
    fn from(value: UserDataRow) -> Self {
        Self {
            id: value.id,
            username: value.username,
            permission: value.permission,
        }
    }
}

#[worker::send]
pub async fn me_user(State(state): State<AppState>, me: UserSession) -> Json<User> {
    let db = &*state.db;
    let res: UserDataRow = db
        .prepare("SELECT * FROM user WHERE id == ?1")
        .bind(&[me.id.into()])
        .unwrap()
        .first(None)
        .await
        .unwrap()
        .unwrap();
    Json(res.into())
}

#[worker::send]
pub async fn new_user(
    State(state): State<AppState>,
    jar: PrivateCookieJar,
    Json(payload): Json<NewUserRequest>,
) -> Result<PrivateCookieJar, (StatusCode, String)> {
    let argon2 = Argon2::default();

    let salt = SaltString::generate(&mut OsRng);

    let password = argon2
        .hash_password(payload.password.as_bytes(), &salt)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error hashing password: {e}"),
            )
        })?;
    let db = &*state.db;

    let prepared = db
        .prepare(
            "INSERT INTO user (username, password, permission) VALUES (?1, ?2, ?3) RETURNING id, permission",
        )
        .bind(&[
            payload.username.into(),
            format!("{password}").into(),
            (PermissionLevel::Verified as u8).into(),
        ])
        .unwrap();
    let inserted_user = prepared
        .first::<InsertedUser>(None)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {e}"),
            )
        })?
        .expect("should have inserted row!");
    set_session(
        jar,
        SessionCookie {
            id: inserted_user.id,
            permission: inserted_user.permission,
        },
    )
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to serialize cookie: {e}"),
        )
    })
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

#[worker::send]
async fn am_i_in(
    session: Result<UserSession, (StatusCode, String)>,
    State(_state): State<AppState>,
) -> &'static str {
    match session {
        Ok(_) => "yep",
        Err(_) => "nope",
    }
}

pub async fn log_out(_session: UserSession, jar: PrivateCookieJar) -> PrivateCookieJar {
    remove_session(jar)
}

#[derive(Clone, Deserialize, Debug)]
struct LoginForm {
    username: String,
    password: String,
}

#[derive(Deserialize, Clone, Debug)]
struct LoginDatabaseResponse {
    id: String,
    password: String,
    permission: PermissionLevel
}

#[worker::send]
async fn log_in(
    State(state): State<AppState>,
    jar: PrivateCookieJar,
    Json(form): Json<LoginForm>,
) -> Result<PrivateCookieJar, (StatusCode, String)> {
    let argon2 = Argon2::default();
    let db = &*state.db;
    let prepared = db.prepare("SELECT id, password, permission FROM user WHERE username == ?1")
        .bind(&[form.username.into()])
        .map_err(|e| {
            (
                StatusCode::UNPROCESSABLE_ENTITY,
                format!("Unprocessable username: {e}."),
            )
        })?;
    let user: LoginDatabaseResponse = prepared.first(None).await.expect("prepared.first should not fail, as it uses no column name").ok_or((StatusCode::UNAUTHORIZED, "Username or password incorrect.".into()))?;
    let password_hash = PasswordHash::new(&user.password).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Database password hash error. Please contact the site owner. {e}")))?;

    argon2.verify_password(form.password.as_bytes(), &password_hash).map_err(|e| match e {
        password_hash::Error::Password => (StatusCode::UNAUTHORIZED, "Username or password incorrect.".into()),
        e => (StatusCode::INTERNAL_SERVER_ERROR, format!("Database password verify error. Please contact the site owner. {e}"))
    })?;
    
    Ok(set_session(jar, SessionCookie { id: user.id, permission: user.permission }).expect("error serializing id and permission level to cookie"))
}
