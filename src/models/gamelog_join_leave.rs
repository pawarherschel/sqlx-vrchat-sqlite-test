use chrono::{DateTime, Utc};

use crate::rows::gamelog_join_leave::GamelogJoinLeaveRow;
use crate::zaphkiel::join_leave_event::JoinLeaveEvent;
use crate::zaphkiel::world_instance::WorldInstance;

/// This is a row from the `gamelog_join_leave` table, but with the `location` field parsed into a
/// `WorldInstance`.
///
/// # Examples
///
/// ```
/// use sqlx_vrchat_sqlite_test::models::gamelog_join_leave::GamelogJoinLeave;
/// use sqlx_vrchat_sqlite_test::rows::gamelog_join_leave::GamelogJoinLeaveRow;
/// use sqlx_vrchat_sqlite_test::zaphkiel::join_leave_event::JoinLeaveEvent;
///
/// let row = GamelogJoinLeave::from(
///     GamelogJoinLeaveRow {
///         id: 1,
///         created_at: chrono::Utc::now(),
///         event: "join".to_string(),
///         display_name: "test".to_string(),
///         location: "wrld_1234:1234".to_string(),
///         user_id: "usr_1234".to_string(),
///         time: 1234,
///     }
/// );
/// assert_eq!(row.id, 1);
/// assert_eq!(row.event, JoinLeaveEvent::Join);
/// assert_eq!(row.display_name, "test");
/// assert_eq!(row.location.clone().unwrap().world_id, "wrld_1234".to_string());
/// assert_eq!(row.location.clone().unwrap().instance_id, "1234".to_string());
/// assert_eq!(row.user_id.clone().unwrap(), "usr_1234".to_string());
/// assert_eq!(row.time.unwrap(), 1234);
/// ```
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
    /// Convert a `GamelogJoinLeaveRow` into a `GamelogJoinLeave`.
    ///
    /// # What it does
    ///
    /// * `id` is copied over.
    /// * `created_at` is copied over.
    /// * `event` is parsed into a `JoinLeaveEvent`.
    /// * `display_name` is copied over.
    /// * `location` is parsed into a `WorldInstance`.
    /// * `user_id` is copied over.
    /// * `time` is checked to see if it's 0 or less. If it is, it's set to `None`. Otherwise, it's
    ///  set to `Some(time as u64)`.
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
    /// Get all rows from the `gamelog_join_leave` table.
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
