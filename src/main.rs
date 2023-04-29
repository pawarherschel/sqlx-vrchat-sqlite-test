use sqlx::migrate::MigrateDatabase;
use sqlx::{Row, Sqlite, SqlitePool};
use sqlx_vrchat_sqlite_test::{AppConfig, GamelogLocation};
use std::error::Error;
use tokio::select;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let settings = AppConfig::get();
    let vrcx_sqlite = settings.vrcx_sqlite;

    if !vrcx_sqlite.exists() {
        panic!("The sqlite3 file does not exist at {:?}", vrcx_sqlite);
    }
    let path_url = if let Some(path_url) = &vrcx_sqlite.to_str() {
        if !Sqlite::database_exists(path_url).await? {
            panic!(
                "Database file exists but the database does not exist at {:?}",
                vrcx_sqlite
            );
        }

        *path_url
    } else {
        panic!("Error converting {path:?} to url", path = vrcx_sqlite);
    };

    dbg!(&vrcx_sqlite);
    dbg!(&path_url);

    let db = SqlitePool::connect(path_url).await?;

    let result = sqlx::query(
        "SELECT name
         FROM sqlite_schema
         WHERE type ='table' 
         AND name NOT LIKE 'sqlite_%';",
    )
    .fetch_all(&db)
    .await
    .unwrap();

    for (idx, row) in result.iter().enumerate() {
        println!("[{}]: {:?}", idx, row.get::<String, &str>("name"));
    }

    let location_results = sqlx::query_as::<_, GamelogLocation>("SELECT * FROM gamelog_location")
        .fetch_all(&db)
        .await?;

    for (idx, row) in location_results.iter().enumerate() {
        println!("[{}]: {:?}", idx, row);
    }

    // println!(
    //     "group name is some:\t{:#?}",
    //     statement
    //         .iter()
    //         .filter(|x| x.group_name.is_some())
    //         .collect::<Vec<&GamelogLocation>>()
    // );
    //
    // println!(
    //     "time is none (only 5): {:#?} \ntotal length: {}",
    //     statement
    //         .iter()
    //         .filter(|x| x.time.is_none())
    //         .take(5)
    //         .collect::<Vec<&GamelogLocation>>(),
    //     statement.len()
    // );

    Ok(())
}
