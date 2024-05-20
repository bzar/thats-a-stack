use crate::AppRouter;
use axum::{
    http::{header, HeaderValue, StatusCode},
    routing::get,
};

macro_rules! get_asset {
    ($file:expr,$mime:expr) => {
        get((
            StatusCode::OK,
            [(header::CONTENT_TYPE, HeaderValue::from_static($mime))],
            include_str!(concat!("../assets/", $file)),
        ))
    };
}

pub fn router() -> AppRouter {
    AppRouter::new()
        .route("/main.css", get_asset!("main.css", "text/css"))
        .route(
            "/htmx.min.js",
            get_asset!("htmx.min.js", "application/javascript"),
        )
        .route("/favicon.svg", get_asset!("favicon.svg", "image/svg+xml"))
}
