use std::error::Error;

use chrono::{DateTime, Utc};
use sqlx::SqlitePool;

/// This is a row from the `gamelog_location` table.
#[derive(
    Clone, PartialEq, Eq, Hash, sqlx::FromRow, Debug, serde::Serialize, serde::Deserialize,
)]
pub struct GamelogLocationRow {
    pub id: i64,
    pub created_at: DateTime<Utc>,
    pub location: String,
    pub world_id: String,
    pub world_name: String,
    pub time: i64,
    pub group_name: String,
}

impl GamelogLocationRow {
    /// Get all rows from the `gamelog_location` table.
    pub async fn get_all(pool: &SqlitePool) -> Result<Vec<GamelogLocationRow>, Box<dyn Error>> {
        Ok(
            sqlx::query_as::<_, GamelogLocationRow>("SELECT * FROM gamelog_location")
                .fetch_all(pool)
                .await?,
        )
    }
}
