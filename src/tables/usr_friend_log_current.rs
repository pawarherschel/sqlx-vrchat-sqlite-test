use crate::models::traits::Table;
use crate::models::trust_level::TrustLevel;
use sqlx::SqlitePool;

#[derive(
    Clone, PartialEq, Eq, Hash, sqlx::FromRow, Debug, serde::Serialize, serde::Deserialize,
)]
pub struct UsrFriendLogCurrent {
    pub user_id: String,
    pub display_name: String,
    pub trust_level: TrustLevel,
}

impl Table for UsrFriendLogCurrent {}

impl UsrFriendLogCurrent {
    pub async fn get_all(
        pool: &SqlitePool,
    ) -> Result<Vec<UsrFriendLogCurrent>, Box<dyn std::error::Error>> {
        let result =
            sqlx::query_as::<_, UsrFriendLogCurrent>("SELECT * FROM usr_friend_log_current")
                .fetch_all(pool)
                .await?;
        Ok(result)
    }
}
