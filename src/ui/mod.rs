use crate::AppRouter;
use askama_axum::{IntoResponse, Template};
use axum::routing::get;

mod counter;

pub fn router() -> AppRouter {
    AppRouter::new()
        .route("/", get(index))
        .nest("/ui", ui_router())
}

fn ui_router() -> AppRouter {
    AppRouter::new().nest("/counter", counter::router())
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {}

async fn index() -> impl IntoResponse {
    IndexTemplate {}
}
