use crate::AppRouter;
use askama_axum::IntoResponse;
use axum::{
    http::{header, HeaderMap, HeaderValue, StatusCode},
    routing::get,
};

static MAIN_CSS: &str = include_str!("../assets/main.css");
static HTMX_JS: &str = include_str!("../assets/htmx.min.js");
static FAVICON: &str = include_str!("../assets/favicon.svg");

pub fn router() -> AppRouter {
    AppRouter::new()
        .route("/main.css", get(main_css))
        .route("/htmx.min.js", get(htmx_js))
        .route("/favicon.svg", get(favicon))
}

fn ok_with_content_type(content: &'static str, content_type: &'static str) -> impl IntoResponse {
    let headers: HeaderMap = [(header::CONTENT_TYPE, HeaderValue::from_static(content_type))]
        .into_iter()
        .collect();
    (StatusCode::OK, headers, content)
}
async fn main_css() -> impl IntoResponse {
    ok_with_content_type(MAIN_CSS, "text/css")
}
async fn htmx_js() -> impl IntoResponse {
    ok_with_content_type(HTMX_JS, "application/javascript")
}
async fn favicon() -> impl IntoResponse {
    ok_with_content_type(FAVICON, "image/svg+xml")
}
