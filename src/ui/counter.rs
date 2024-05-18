use askama_axum::{IntoResponse, Template};
use axum::{
    extract::State,
    routing::{get, post},
};

use crate::{AppRouter, AppState};

#[derive(Template)]
#[template(source = "<span>{{ count }}</span>", ext = "txt")]
struct CounterTemplate {
    count: i64,
}

pub fn router() -> AppRouter {
    AppRouter::new()
        .route("/", get(current))
        .route("/increment", post(increment))
}

async fn current(State(state): State<AppState>) -> impl IntoResponse {
    CounterTemplate {
        count: state.counter().await,
    }
}

async fn increment(State(mut state): State<AppState>) -> impl IntoResponse {
    CounterTemplate {
        count: state.increment_counter().await,
    }
}
