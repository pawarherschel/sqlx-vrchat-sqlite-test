use itertools::Itertools;
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

use sqlx_vrchat_sqlite_test::models::app_config::AppConfig;
use sqlx_vrchat_sqlite_test::models::connection::establish_connection;
use sqlx_vrchat_sqlite_test::models::gamelog_join_leave::GamelogJoinLeave;
use sqlx_vrchat_sqlite_test::models::gamelog_location::GamelogLocation;
use sqlx_vrchat_sqlite_test::models::usr_friend_log_current::UsrFriendLogCurrent;
use sqlx_vrchat_sqlite_test::zaphkiel::join_leave_event::JoinLeaveEvent;
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

    loop {
        let mut input = String::new();
        println!("Enter a user to search for: ");
        std::io::stdin().read_line(&mut input)?;
        let input = input.as_str().trim();
        if input == "exit" {
            break;
        }
        let input = input.trim().to_string();
        println!("Searching for user: {}", input);
        let user = all_friends
            .iter()
            .filter(|&x| x.display_name == input)
            .collect::<Vec<_>>();
        let user = user.clone().into_iter().next();
        if user.is_none() {
            println!("User not found");
            continue;
        } else {
            println!("User found: {:#?}", user.unwrap());
        }
        let user = user.unwrap();
        println!("Searching for all locations where user joined: ");
        all_join_leave
            .iter()
            .filter(|x| x.display_name == user.display_name)
            .filter(|x| x.event == JoinLeaveEvent::Join)
            .filter(|x| x.location.is_some())
            .map(|x| x.location.as_ref().unwrap().world_id.clone())
            .map(|x| {
                all_gamelog_locations
                    .iter()
                    .find(|y| y.world_instance.world_id == x)
                    .unwrap()
                    .world_name
                    .clone()
            })
            .unique()
            .for_each(|x| println!("{}", x));
    }

    Ok(())
}
