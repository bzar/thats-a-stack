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
    pub async fn counter(&self) -> domain::Counter {
        sqlx::query!("SELECT value FROM count LIMIT 1")
            .fetch_one(&self.pool)
            .await
            .map(|row| row.value)
            .unwrap_or(0)
    }
    pub async fn increment_counter(&mut self) -> domain::Counter {
        let current = sqlx::query!("SELECT id, value FROM count LIMIT 1")
            .fetch_one(&self.pool)
            .await;
        if let Ok(row) = current {
            let count = row.value + 1;
            sqlx::query!("UPDATE count SET value = $2 where id = $1", row.id, count)
                .execute(&self.pool)
                .await
                .unwrap();
            count
        } else {
            sqlx::query!("INSERT INTO count(value) VALUES ($1)", 1)
                .execute(&self.pool)
                .await
                .unwrap();
            1
        }
    }
}
