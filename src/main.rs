use sqlx::migrate::MigrateDatabase;
use sqlx::Sqlite;
use sqlx_vrchat_sqlite_test::{AppConfig, GamelogLocation};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let settings = AppConfig::get();
    let vrcx_sqlite = settings.vrcx_sqlite;

    if !vrcx_sqlite.exists() {
        panic!("The sqlite3 file does not exist at {:?}", vrcx_sqlite);
    }
    if let Some(path_url) = &vrcx_sqlite.to_str() {
        if !Sqlite::database_exists(path_url).await? {
            panic!(
                "Database file exists but the database does not exist at {:?}",
                vrcx_sqlite
            );
        }
    } else {
        panic!("Error converting {path:?} to url", path = vrcx_sqlite);
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
