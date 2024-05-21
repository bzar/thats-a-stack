use crate::domain;
use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct AppState {
    pool: PgPool,
}

impl AppState {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    pub async fn counter(&self, id: domain::CounterId) -> domain::Counter {
        sqlx::query!("SELECT value FROM count WHERE id = $1", id)
            .fetch_one(&self.pool)
            .await
            .map(|row| row.value)
            .unwrap_or(0)
    }
    pub async fn increment_counter(&mut self, id: domain::CounterId) -> domain::Counter {
        sqlx::query!(
            "INSERT INTO count(id, value) VALUES ($1, 1)
             ON CONFLICT (id)
             DO UPDATE SET value = count.value + 1
             RETURNING value",
            id
        )
        .fetch_one(&self.pool)
        .await
        .unwrap()
        .value
    }
}
