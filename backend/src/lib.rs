use axum::{
    http::{HeaderValue, Method},
    routing::get,
    Router,
};
use tower_http::cors::CorsLayer;
use tower_service::Service;
use worker::*;

fn router() -> Router {
    Router::new()
        .layer(
            CorsLayer::new()
                .allow_origin(
                    "https://api.bot.leifbarton.dev"
                        .parse::<HeaderValue>()
                        .unwrap(),
                )
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::OPTIONS,
                    Method::PUT,
                    Method::PATCH,
                    Method::DELETE,
                    Method::HEAD,
                ]),
        )
        .route("/", get(root))
}

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    _env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    Ok(router().call(req).await?)
}

pub async fn root() -> &'static str {
    "hello!"
}
