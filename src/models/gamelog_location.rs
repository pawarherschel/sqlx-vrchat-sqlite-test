use chrono::{DateTime, Utc};

use crate::rows::gamelog_location::GamelogLocationRow;
use crate::zaphkiel::world_instance::WorldInstance;

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, Default)]
pub struct GamelogLocation {
    pub id: i64,
    pub created_at: DateTime<Utc>,
    pub world_name: String,
    pub world_instance: WorldInstance,
    pub time: Option<u64>,
    pub group_name: Option<String>,
}

impl GamelogLocation {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
}

impl GamelogLocation {
    pub async fn get_all(
        pool: &sqlx::SqlitePool,
    ) -> Result<Vec<GamelogLocation>, Box<dyn std::error::Error>> {
        Ok(GamelogLocationRow::get_all(pool)
            .await?
            .into_iter()
            .map(|x| x.into())
            .collect())
    }
}

impl From<GamelogLocationRow> for GamelogLocation {
    fn from(row: GamelogLocationRow) -> Self {
        let world_id = row.location;
        let mut ret = Self::new();
        ret.world_instance = WorldInstance::from(world_id);

        ret.id = row.id;
        ret.created_at = row.created_at;
        ret.world_name = row.world_name.trim().to_string();
        ret.time = match row.time {
            ..=0 => None,
            _ => Some(row.time as u64),
        };
        ret.group_name = match row.group_name {
            x if x.is_empty() => None,
            _ => Some(row.group_name),
        };

        ret
    }
}
