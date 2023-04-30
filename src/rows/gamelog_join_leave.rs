use std::error::Error;

use chrono::{DateTime, Utc};
use sqlx::SqlitePool;

#[derive(
    Clone, PartialEq, Eq, Hash, sqlx::FromRow, Debug, serde::Serialize, serde::Deserialize,
)]
pub struct GamelogJoinLeaveRow {
    pub id: i64,
    pub created_at: DateTime<Utc>,
    #[sqlx(rename = "type")]
    pub event: String,
    pub display_name: String,
    pub location: String,
    pub user_id: String,
    pub time: i64,
}

impl GamelogJoinLeaveRow {
    pub async fn get_all(pool: &SqlitePool) -> Result<Vec<GamelogJoinLeaveRow>, Box<dyn Error>> {
        Ok(
            sqlx::query_as::<_, GamelogJoinLeaveRow>("SELECT * FROM gamelog_join_leave")
                .fetch_all(pool)
                .await?,
        )
    }
}
