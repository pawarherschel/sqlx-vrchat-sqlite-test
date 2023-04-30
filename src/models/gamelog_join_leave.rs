use crate::rows::gamelog_join_leave::GamelogJoinLeaveRow;
use crate::zaphkiel::join_leave_event::JoinLeaveEvent;
use crate::zaphkiel::world_instance::WorldInstance;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, Default)]
pub struct GamelogJoinLeave {
    pub id: i64,
    pub created_at: DateTime<Utc>,
    pub event: JoinLeaveEvent,
    pub display_name: String,
    pub location: Option<WorldInstance>,
    pub user_id: Option<String>,
    pub time: Option<u64>,
}

impl GamelogJoinLeave {
    pub fn new() -> Self {
        Self::default()
    }
}

impl From<GamelogJoinLeaveRow> for GamelogJoinLeave {
    fn from(row: GamelogJoinLeaveRow) -> Self {
        let mut ret = Self::new();
        ret.id = row.id;
        ret.created_at = row.created_at;
        ret.event = row.event.parse().unwrap();
        ret.display_name = row.display_name;
        ret.location = if let Ok(location) = row.location.parse() {
            Some(location)
        } else {
            None
        };
        ret.user_id = match row.user_id {
            x if x.is_empty() => None,
            _ => Some(row.user_id),
        };
        ret.time = match row.time {
            ..=0 => None,
            _ => Some(row.time as u64),
        };

        ret
    }
}

impl GamelogJoinLeave {
    pub async fn get_all(
        pool: &sqlx::SqlitePool,
    ) -> Result<Vec<GamelogJoinLeave>, Box<dyn std::error::Error>> {
        Ok(GamelogJoinLeaveRow::get_all(pool)
            .await?
            .into_iter()
            .map(|x| x.into())
            .collect())
    }
}
