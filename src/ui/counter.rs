use askama_axum::{IntoResponse, Template};
use axum::{
    extract::State,
    routing::{get, post},
};

use crate::{domain, AppRouter, AppState};

#[derive(Template)]
#[template(source = "<span>{{ count }}</span>", ext = "txt")]
struct CounterTemplate {
    count: domain::Counter,
}

pub fn router() -> AppRouter {
    AppRouter::new()
        .route("/", get(current))
        .route("/increment", post(increment))
}

async fn current(State(state): State<AppState>) -> impl IntoResponse {
    CounterTemplate {
        count: state.counter(0).await,
    }
}

async fn increment(State(mut state): State<AppState>) -> impl IntoResponse {
    CounterTemplate {
        count: state.increment_counter(0).await,
    }
}
