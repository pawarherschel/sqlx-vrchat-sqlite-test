// use crate::models::traits::Table;
// use crate::rows::gamelog_location::GamelogLocation;
// use crate::rows::sqlite_master::SqliteMaster;
// use crate::rows::usr_friend_log_current::UsrFriendLogCurrent;
// use crate::rows::Tables;
// use sqlx::SqlitePool;
// use std::error::Error;
//
// pub async fn get_all(
//     table: Tables,
//     pool: &SqlitePool,
// ) -> Result<Vec<Box<impl Table>>, Box<dyn Error>> {
//     match table {
//         Tables::SqliteMaster => Ok(sqlx::query_as::<_, SqliteMaster>(
//             "SELECT * FROM sqlite_master",
//         )
//         .fetch_all(pool)
//         .await?),
//
//         Tables::GamelogLocation => Ok(sqlx::query_as::<_, GamelogLocation>(
//             "SELECT * FROM gamelog_location",
//         )
//         .fetch_all(pool)
//         .await?),
//
//         Tables::UsrFriendLogCurrent(usr_id) => Ok(sqlx::query_as::<_, UsrFriendLogCurrent>(
//             "SELECT * FROM usr(1)_friend_log_current",
//         )
//         .bind(&usr_id)
//         .fetch_all(pool)
//         .await?),
//     }
// }
