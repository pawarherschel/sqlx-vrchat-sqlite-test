use std::error::Error;

use sqlx_vrchat_sqlite_test::models::app_config::AppConfig;
use sqlx_vrchat_sqlite_test::models::connection::establish_connection;
use sqlx_vrchat_sqlite_test::models::gamelog_join_leave::GamelogJoinLeave;
use sqlx_vrchat_sqlite_test::models::gamelog_location::GamelogLocation;
use sqlx_vrchat_sqlite_test::models::usr_friend_log_current::UsrFriendLogCurrent;
use sqlx_vrchat_sqlite_test::zaphkiel::trust_level::TrustLevel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let settings = AppConfig::get().build().await?;
    let db = establish_connection(settings.url).await?;

    let all_gamelog_locations = GamelogLocation::get_all(&db).await?;

    // println!(
    //     "all_gamelog_locations group name is some:\t{:#?}",
    //     _all_gamelog_locations
    //         .iter()
    //         .filter(|x| x.group_name.is_some())
    //         .collect::<Vec<&GamelogLocation>>()
    // );

    println!(
        "Number of GamelogLocation rows: {}",
        all_gamelog_locations.len()
    );
    println!(
        "Number of GamelogLocations with group_name is some: {}",
        all_gamelog_locations
            .iter()
            .filter(|x| x.group_name.is_some())
            .count()
    );

    let all_friends = UsrFriendLogCurrent::get_all(&db, &settings.usr_id).await?;

    println!("Number of UsrFriendLogCurrent rows: {}", all_friends.len());

    println!(
        "Number of UsrFriendLogCurrent with trust level is user: {}",
        all_friends
            .iter()
            .filter(|x| x.trust_level == TrustLevel::User)
            .count()
    );

    println!(
        "Number of UsrFriendLogCurrent with trust level is unknown: {}",
        all_friends
            .iter()
            .filter(|x| x.trust_level == TrustLevel::Unknown)
            .count()
    );

    // println!(
    //     "friends with trust level user:\t{:#?}",
    //     all_friends
    //         .iter()
    //         .filter(|x| x.trust_level == TrustLevel::User)
    //         .collect::<Vec<&UsrFriendLogCurrent>>()
    // );

    // println!(
    //     "friends with trust level unknown:\t{:#?}",
    //     all_friends
    //         .iter()
    //         .filter(|x| x.trust_level == TrustLevel::Unknown)
    //         .collect::<Vec<&UsrFriendLogCurrent>>()
    // );

    let all_join_leave = GamelogJoinLeave::get_all(&db).await?;

    println!("Number of GamelogJoinLeave rows: {}", all_join_leave.len());

    println!(
        "Number of GamelogJoinLeave with location is none: {}",
        all_join_leave
            .iter()
            .filter(|x| x.location.is_none())
            .count()
    );

    println!(
        "Number of GamelogJoinLeave with user is none: {}",
        all_join_leave
            .iter()
            .filter(|x| x.user_id.is_none())
            .count()
    );

    Ok(())
}
