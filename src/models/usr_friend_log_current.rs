use sqlx::SqlitePool;

use crate::rows::usr_friend_log_current::UsrFriendLogCurrentRow;
use crate::zaphkiel::trust_level::TrustLevel;

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, Default)]
pub struct UsrFriendLogCurrent {
    pub user_id: String,
    pub display_name: String,
    pub trust_level: TrustLevel,
}

impl UsrFriendLogCurrent {
    #[allow(dead_code)]
    fn new() -> Self {
        Self::default()
    }
}

impl UsrFriendLogCurrent {
    pub async fn get_all(
        pool: &SqlitePool,
        user_id: &str,
    ) -> Result<Vec<UsrFriendLogCurrent>, Box<dyn std::error::Error>> {
        Ok(UsrFriendLogCurrentRow::get_all(pool, user_id)
            .await?
            .into_iter()
            .map(|x| x.into())
            .collect())
    }
}

impl From<UsrFriendLogCurrentRow> for UsrFriendLogCurrent {
    fn from(row: UsrFriendLogCurrentRow) -> Self {
        Self {
            user_id: row.user_id,
            display_name: row.display_name,
            trust_level: TrustLevel::from(row.trust_level),
        }
    }
}
