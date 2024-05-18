use sqlx::{AnyPool, Row};

#[derive(Debug, Clone)]
pub struct AppState {
    pool: AnyPool,
}

impl AppState {
    pub fn new(pool: AnyPool) -> Self {
        Self { pool }
    }
    pub async fn counter(&self) -> i64 {
        sqlx::query("SELECT value FROM count LIMIT 1")
            .fetch_one(&self.pool)
            .await
            .map(|row| row.get(0))
            .unwrap_or(0)
    }
    pub async fn increment_counter(&mut self) -> i64 {
        let current = sqlx::query("SELECT id, value FROM count LIMIT 1")
            .fetch_one(&self.pool)
            .await;
        if let Ok(row) = current {
            let id: i64 = row.get(0);
            let value: i64 = row.get(1);
            sqlx::query("UPDATE count SET value = $2 where id = $1")
                .bind(id)
                .bind(value + 1)
                .execute(&self.pool)
                .await
                .unwrap();
            value + 1
        } else {
            sqlx::query("INSERT INTO count(value) VALUES ($1)")
                .bind(0)
                .execute(&self.pool)
                .await
                .unwrap();
            0
        }
    }
}
