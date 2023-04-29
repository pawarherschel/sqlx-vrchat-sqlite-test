use sqlx::Row;
use sqlx_vrchat_sqlite_test::models::app_config::AppConfig;
use sqlx_vrchat_sqlite_test::models::connection::establish_connection;
use sqlx_vrchat_sqlite_test::tables::gamelog_location::GamelogLocation;
use sqlx_vrchat_sqlite_test::tables::Tables;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let settings = AppConfig::get().build().await?;
    let db = establish_connection(settings.url).await?;

    let all_gamelog_locations = GamelogLocation::get_all(&db).await?;

    println!(
        "group name is some:\t{:#?}",
        all_gamelog_locations
            .iter()
            .filter(|x| x.group_name.is_some())
            .collect::<Vec<&GamelogLocation>>()
    );

    println!(
        "time is none (only 5): {:#?} \ntotal length: {}",
        all_gamelog_locations
            .iter()
            .filter(|x| x.time.is_none())
            .take(5)
            .collect::<Vec<&GamelogLocation>>(),
        all_gamelog_locations.len()
    );

    Ok(())
}
