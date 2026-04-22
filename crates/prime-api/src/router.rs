use crate::{handlers, state::ApiState};
use axum::{routing::{get, post}, Router, response::Html};

async fn dashboard() -> Html<&'static str> {
    Html(crate::dashboard::html())
}

pub fn build_router(state: ApiState) -> Router {
    Router::new()
        .route("/",                           get(dashboard))
        .route("/dashboard",                  get(dashboard))
        .route("/health",                     get(handlers::health))
        .route("/status",                     get(handlers::status))
        .route("/v1/chat/completions",        post(handlers::chat_completions))
        .with_state(state)
}
