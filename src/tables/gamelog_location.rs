use crate::models::traits::Table;
use crate::tables::Tables;
use chrono::{DateTime, Utc};
use sqlx::SqlitePool;
use std::error::Error;

#[derive(
    Clone, PartialEq, Eq, Hash, sqlx::FromRow, Debug, serde::Serialize, serde::Deserialize,
)]
pub struct GamelogLocation {
    pub id: i64,
    pub created_at: DateTime<Utc>,
    pub location: String,
    pub world_id: String,
    pub world_name: String,
    pub time: Option<i64>,
    pub group_name: Option<String>,
}

impl Table for GamelogLocation {}

impl GamelogLocation {
    pub async fn get_all(pool: &SqlitePool) -> Result<Vec<GamelogLocation>, Box<dyn Error>> {
        Ok(
            sqlx::query_as::<_, GamelogLocation>("SELECT * FROM gamelog_location")
                .fetch_all(pool)
                .await?
                .into_iter()
                .map(|x| {
                    let mut result = x.clone();
                    if let Some(time) = x.time {
                        if time == 0 {
                            result.time = None;
                        }
                    }
                    if let Some(group_name) = &x.group_name {
                        if group_name.is_empty() {
                            result.group_name = None;
                        }
                    }

                    result
                })
                .collect(),
        )
    }
}
