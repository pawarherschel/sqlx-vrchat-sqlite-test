use std::error::Error;

use sqlx_vrchat_sqlite_test::models::app_config::AppConfig;
use sqlx_vrchat_sqlite_test::models::connection::establish_connection;
use sqlx_vrchat_sqlite_test::models::gamelog_location::GamelogLocation;
use sqlx_vrchat_sqlite_test::models::usr_friend_log_current::UsrFriendLogCurrent;
use sqlx_vrchat_sqlite_test::rows::usr_friend_log_current::UsrFriendLogCurrentRow;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let settings = AppConfig::get().build().await?;
    let db = establish_connection(settings.url).await?;

    let all_gamelog_locations = GamelogLocation::get_all(&db).await?;

    println!(
        "all_gamelog_locations group name is some:\t{:#?}",
        all_gamelog_locations
            .iter()
            .filter(|x| x.group_name.is_some())
            .collect::<Vec<&GamelogLocation>>()
    );

    let all_friends = UsrFriendLogCurrent::get_all(&db, &settings.usr_id).await?;

    println!("all_friends:\t{:#?}", all_friends);

    Ok(())
}
