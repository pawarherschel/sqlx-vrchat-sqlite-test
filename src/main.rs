use sqlx_vrchat_sqlite_test::{AppConfig, GamelogLocation};

#[tokio::main]
async fn main() -> sqlite::Result<()> {
    let settings = AppConfig::get();
    let vrcx_sqlite = settings.vrcx_sqlite;

    let connection = sqlite::open(vrcx_sqlite)?;

    let query = "SELECT * FROM gamelog_location";

    let statement = connection
        .prepare(query)?
        .into_iter()
        .map(|row| {
            let row = row.unwrap();

            let id = row.try_read::<i64, _>("id").unwrap_or(-1);
            let created_at = row
                .try_read::<&str, _>("created_at")
                .unwrap_or_default()
                .to_string();
            let location = row
                .try_read::<&str, _>("location")
                .unwrap_or_default()
                .to_string();
            let world_id = row
                .try_read::<&str, _>("world_id")
                .unwrap_or_default()
                .to_string();
            let world_name = row
                .try_read::<&str, _>("world_name")
                .unwrap_or_default()
                .to_string();
            let time = match row.try_read::<i64, _>("time").unwrap_or_default() {
                something if something == 0 => None,
                something => Some(something),
            };
            let group_name = match row.try_read::<&str, _>("group_name").unwrap_or_default() {
                "" => None,
                something => Some(something.to_string()),
            };

            GamelogLocation {
                id,
                created_at,
                location,
                world_id,
                world_name,
                time,
                group_name,
            }
        })
        .collect::<Vec<GamelogLocation>>();

    println!(
        "group name is some:\t{:#?}",
        statement
            .iter()
            .filter(|x| x.group_name.is_some())
            .collect::<Vec<&GamelogLocation>>()
    );

    println!(
        "time is none (only 5): {:#?} \ntotal length: {}",
        statement
            .iter()
            .filter(|x| x.time.is_none())
            .take(5)
            .collect::<Vec<&GamelogLocation>>(),
        statement.len()
    );

    Ok(())
}
