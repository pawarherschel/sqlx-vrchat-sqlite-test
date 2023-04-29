use sqlx::SqlitePool;
use std::error::Error;

pub async fn establish_connection(url: Option<String>) -> Result<SqlitePool, Box<dyn Error>> {
    let url = url.unwrap_or_else(|| {
        panic!("Error: No url provided. Please provide a url in the config file or as an argument.")
    });
    let url = url.as_str();
    let pool = SqlitePool::connect(url).await?;

    Ok(pool)
}
