use sqlx::SqlitePool;

#[derive(
    Clone, PartialEq, Eq, Hash, sqlx::FromRow, Debug, serde::Serialize, serde::Deserialize,
)]
pub struct UsrFriendLogCurrentRow {
    pub user_id: String,
    pub display_name: String,
    pub trust_level: String,
}

impl UsrFriendLogCurrentRow {
    pub async fn get_all(
        pool: &SqlitePool,
        user_id: &str,
    ) -> Result<Vec<UsrFriendLogCurrentRow>, Box<dyn std::error::Error>> {
        let q = format!("SELECT * FROM usr{}_friend_log_current", user_id);
        let result = sqlx::query_as::<_, UsrFriendLogCurrentRow>(&q)
            .bind(user_id)
            .fetch_all(pool)
            .await?;
        Ok(result)
    }
}
