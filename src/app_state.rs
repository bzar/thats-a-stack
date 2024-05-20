use crate::domain;
use sqlx::{PgPool, Row};

#[derive(Debug, Clone)]
pub struct AppState {
    pool: PgPool,
}

impl AppState {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    pub async fn counter(&self) -> domain::Counter {
        sqlx::query("SELECT value FROM count LIMIT 1")
            .fetch_one(&self.pool)
            .await
            .map(|row| row.get(0))
            .unwrap_or_else(|_| Default::default())
    }
    pub async fn increment_counter(&mut self) -> domain::Counter {
        let current = sqlx::query("SELECT id, value FROM count LIMIT 1")
            .fetch_one(&self.pool)
            .await;
        if let Ok(row) = current {
            let id: i32 = row.get(0);
            let count: i32 = row.get(1);
            let new_count = count + 1;
            sqlx::query("UPDATE count SET value = $2 where id = $1")
                .bind(id)
                .bind(new_count)
                .execute(&self.pool)
                .await
                .unwrap();
            new_count
        } else {
            sqlx::query("INSERT INTO count(value) VALUES ($1)")
                .bind(0)
                .execute(&self.pool)
                .await
                .unwrap();
            Default::default()
        }
    }
}
