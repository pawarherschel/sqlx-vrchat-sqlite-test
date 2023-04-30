use std::str::FromStr;

use crate::zaphkiel::world_regions::Regions;

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, Default)]
pub struct WorldInstance {
    pub world_id: String,
    pub instance_id: String,
    pub nonce: Option<String>,
    pub hidden: Option<String>,
    pub private: Option<String>,
    pub region: Option<Regions>,
    pub friends: Option<String>,
    pub group: Option<String>,
}

impl WorldInstance {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    sqlx::Type,
    Default,
)]
pub enum WorldInstanceParseError {
    Empty,
    InvalidFormat,
    InvalidWorldId,
    InvalidInstanceId,
    InvalidOptionalField,
    #[default]
    Other,
}

impl FromStr for WorldInstance {
    type Err = WorldInstanceParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(WorldInstanceParseError::Empty);
        }

        let mut ret = Self::new();

        let world_id = s;
        let parts = world_id.split(':').collect::<Vec<_>>();
        if parts.len() != 2 {
            return Err(WorldInstanceParseError::InvalidFormat);
        }

        if parts[0].is_empty() {
            return Err(WorldInstanceParseError::InvalidWorldId);
        }
        ret.world_id = parts[0].to_string();

        let parts = parts[1].split('~').collect::<Vec<_>>();
        if parts[0].is_empty() {
            return Err(WorldInstanceParseError::InvalidInstanceId);
        }
        ret.instance_id = parts[0].to_string();

        for part in parts {
            let parts = part.split('(').collect::<Vec<_>>();
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

        Ok(ret)
    }
}

impl From<&str> for WorldInstance {
    fn from(s: &str) -> Self {
        Self::from_str(s).unwrap()
    }
}

impl From<String> for WorldInstance {
    fn from(s: String) -> Self {
        Self::from_str(&s).unwrap()
    }
}
