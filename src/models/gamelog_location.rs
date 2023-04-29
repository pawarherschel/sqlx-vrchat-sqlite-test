use chrono::{DateTime, Utc};

use crate::rows::gamelog_location::GamelogLocationRow;
use crate::zaphkiel::world_regions::Regions;

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, Default)]
pub struct GamelogLocation {
    pub id: i64,
    pub created_at: DateTime<Utc>,
    pub world_name: String,
    pub world_id: String,
    pub instance_id: String,
    pub nonce: Option<String>,
    pub hidden: Option<String>,
    pub private: Option<String>,
    pub region: Option<Regions>,
    pub friends: Option<String>,
    pub group: Option<String>,
    pub time: Option<i64>,
    pub group_name: Option<String>,
}

impl GamelogLocation {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
}

impl From<GamelogLocationRow> for GamelogLocation {
    fn from(row: GamelogLocationRow) -> Self {
        // dbg!(&row);
        let world_id = row.location;
        // dbg!(&world_id);
        let mut ret = Self::new();

        let parts = world_id.split(':').collect::<Vec<_>>();
        // dbg!(&parts);
        ret.world_id = parts[0].to_string();

        let parts = parts[1].split('~').collect::<Vec<_>>();
        ret.instance_id = parts[0].to_string();

        for part in parts {
            let parts = part.split('(').collect::<Vec<_>>();
            // dbg!(&parts);
            let key = parts[0];
            if parts.len() < 2 {
                continue;
            }
            let value = parts[1].split(')').collect::<Vec<_>>()[0].to_string();

            match key {
                "nonce" => ret.nonce = Some(value),
                "hidden" => ret.hidden = Some(value),
                "private" => ret.private = Some(value),
                "region" => ret.region = Some(value.into()),
                "friends" => ret.friends = Some(value),
                "group" => ret.group = Some(value),
                _ => panic!("Unknown key: {}", key),
            }
        }

        ret.id = row.id;
        ret.created_at = row.created_at;
        ret.world_name = row.world_name.trim().to_string();
        ret.time = match row.time {
            Some(0) => None,
            _ => row.time,
        };
        ret.group_name = match row.group_name {
            Some(x) if x.is_empty() => None,
            _ => row.group_name,
        };

        ret
    }
}

// .into_iter()
// .map(|x| {
// let mut result = x.clone();
// if let Some(time) = x.time {
// if time == 0 {
// result.time = None;
// }
// }
// if let Some(group_name) = &x.group_name {
// if group_name.is_empty() {
// result.group_name = None;
// }
// }
//
// result
// })
// .collect()
