use sqlx::SqlitePool;
use std::error::Error;

/// This is a row from the `usr_friend_log_current` table.
///
/// Example:
/// ```
/// use sqlx_vrchat_sqlite_test::models::usr_friend_log_current::UsrFriendLogCurrent;
/// use sqlx_vrchat_sqlite_test::zaphkiel::trust_level::TrustLevel;
///
/// let row = UsrFriendLogCurrent {
///    user_id: "usr_12345678-1234-1234-1234-123456789abc".to_string(),
///   display_name: "Some User".to_string(),
///  trust_level: TrustLevel::User,
/// };
/// ```
#[derive(
    Clone, PartialEq, Eq, Hash, sqlx::FromRow, Debug, serde::Serialize, serde::Deserialize,
)]
pub struct UsrFriendLogCurrentRow {
    pub user_id: String,
    pub display_name: String,
    pub trust_level: String,
}

impl UsrFriendLogCurrentRow {
    /// Get all rows from the `usr_friend_log_current` table.
    pub async fn get_all(
        pool: &SqlitePool,
        user_id: &str,
    ) -> Result<Vec<UsrFriendLogCurrentRow>, Box<dyn Error>> {
        let q = format!("SELECT * FROM usr{}_friend_log_current", user_id);
        let result = sqlx::query_as::<_, UsrFriendLogCurrentRow>(&q)
            .bind(user_id)
            .fetch_all(pool)
            .await?;
        Ok(result)
    }
}
