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
    // Optionally load environment from .env
    dotenv::dotenv().ok();

    // Initialize logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or("debug".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Connect to database
    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL must be defined in environment");
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Could not connect to database");

    // Run migrations if needed
    sqlx::migrate!().run(&pool).await.unwrap();

    // Initialize application state
    let state = AppState::new(pool);

    // Initialize routes
    let app = AppRouter::new()
        .nest("/", ui::router())
        .nest("/assets", assets::router())
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .with_state(state);

    // Serve application
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_owned());
    let port: u16 = env::var("PORT")
        .map(|p| p.parse().expect("Port must be an integer"))
        .unwrap_or(3000);
    let listener = tokio::net::TcpListener::bind((host, port)).await.unwrap();

    tracing::info!("listening on {:?}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
