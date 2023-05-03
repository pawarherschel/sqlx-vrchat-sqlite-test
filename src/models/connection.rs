use std::error::Error;

use sqlx::SqlitePool;

/// Establish a connection to the database.
///
/// # Arguments
///
/// - `url` - The url to the database. If `None`, then the url from the config file will be used.
pub async fn establish_connection(url: Option<String>) -> Result<SqlitePool, Box<dyn Error>> {
    let url = url.unwrap_or_else(|| {
        panic!("Error: No url provided. Please provide a url in the config file or as an argument.")
    });
    let url = url.as_str();
    let pool = SqlitePool::connect(url).await?;

    Ok(pool)
}
