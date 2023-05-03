use chrono::{DateTime, Utc};

use crate::rows::gamelog_location::GamelogLocationRow;
use crate::zaphkiel::world_instance::WorldInstance;

/// This is a row from the `gamelog_location` table, but with the `location` field parsed into a
/// `WorldInstance`.
///
/// # Examples
///
/// ```
/// use sqlx_vrchat_sqlite_test::models::gamelog_location::GamelogLocation;
/// use sqlx_vrchat_sqlite_test::rows::gamelog_location::GamelogLocationRow;
///
/// let row = GamelogLocation::from(
///     GamelogLocationRow {
///         id: 1,
///         created_at: chrono::Utc::now(),
///         location: "wrld_1234:1234".to_string(),
///         world_id: "wrld_1234".to_string(),
///         world_name: "test".to_string(),
///         time: 1234,
///         group_name: "test".to_string(),
///     }
/// );
/// assert_eq!(row.id, 1);
/// assert_eq!(row.world_instance.world_id, "wrld_1234".to_string());
/// assert_eq!(row.world_instance.instance_id, "1234".to_string());
/// assert_eq!(row.world_name, "test");
/// assert_eq!(row.time.clone().unwrap(), 1234);
/// assert_eq!(row.group_name.clone().unwrap(), "test".to_string());
/// ```
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
    /// Create a new `GamelogLocation` by calling `GamelogLocation::default()`.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
}

impl GamelogLocation {
    /// Get all rows from the `gamelog_location` table.
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
    /// Convert a `GamelogLocationRow` into a `GamelogLocation`.
    ///
    /// # What it does
    ///
    /// * `location` is parsed into a `WorldInstance`.
    /// * `id` is copied.
    /// * `created_at` is copied.
    /// * `world_name` is copied.
    /// * `time` is copied, but if it is `0` or less, it is set to `None`.
    /// * `group_name` is copied, but if it is empty, it is set to `None`.
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
