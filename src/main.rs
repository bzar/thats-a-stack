use std::env;

use axum::Router;
use sqlx::PgPool;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod app_state;
mod assets;
mod domain;
mod ui;

pub use app_state::AppState;
pub type AppRouter = Router<AppState>;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or("debug".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    sqlx::any::install_default_drivers();
    let database_url = env::var("DATABASE_URL").unwrap();
    let pool = PgPool::connect(&database_url).await.unwrap();
    sqlx::migrate!().run(&pool).await.unwrap();
    let state = AppState::new(pool);

    let app = AppRouter::new()
        .nest("/", ui::router())
        .nest("/assets", assets::router())
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .with_state(state);

    let host = env::var("HOST").unwrap_or("0.0.0.0".to_owned());
    let port = env::var("PORT").unwrap_or("3000".to_owned());
    let listener = tokio::net::TcpListener::bind(format!("{host}:{port}"))
        .await
        .unwrap();

    tracing::info!("listening on {:?}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
