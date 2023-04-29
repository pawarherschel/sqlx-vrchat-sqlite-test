use sqlx::SqlitePool;

#[derive(
    Clone, PartialEq, Eq, Hash, sqlx::FromRow, Debug, serde::Serialize, serde::Deserialize,
)]
pub struct SqliteMaster {
    pub type_: String,
    pub name: String,
    pub tbl_name: String,
    pub rootpage: i32,
    pub sql: String,
}

impl SqliteMaster {
    pub async fn get_all(
        pool: &SqlitePool,
    ) -> Result<Vec<SqliteMaster>, Box<dyn std::error::Error>> {
        let result = sqlx::query_as::<_, SqliteMaster>("SELECT * FROM sqlite_master")
            .fetch_all(pool)
            .await?;
        Ok(result)
    }
}
