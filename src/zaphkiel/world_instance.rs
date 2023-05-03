use std::str::FromStr;

use crate::zaphkiel::world_regions::Regions;

/// A struct representing a world instance.
///
/// # Examples
///
/// ```
/// use std::str::FromStr;
/// use sqlx_vrchat_sqlite_test::zaphkiel::world_instance::WorldInstance;
///
/// let world_instance = WorldInstance::from_str("wrld_1234:1234~private(usr_1234)").unwrap();
/// assert_eq!(world_instance.world_id, "wrld_1234");
/// assert_eq!(world_instance.instance_id, "1234");
/// assert_eq!(world_instance.private, Some("usr_1234".to_string()));
/// ```
///
/// ```
/// use std::str::FromStr;
/// use sqlx_vrchat_sqlite_test::zaphkiel::world_instance::WorldInstance;
///
/// let world_instance = WorldInstance::from_str("wrld_1234:1234").unwrap();
/// assert_eq!(world_instance.world_id, "wrld_1234");
/// assert_eq!(world_instance.instance_id, "1234");
/// assert_eq!(world_instance.private, None);
/// ```
///
/// # Member variables:
///
/// - `world_id`: The world id of the world instance.
/// - `instance_id`: The instance id of the world instance.
/// - `nonce`: The optional nonce of the world instance.
/// - `hidden`: The optional hidden of the world instance.
/// - `private`: The optional private of the world instance.
/// - `region`: The optional region of the world instance.
///
/// # Creating a new `WorldInstance`:
/// ```
/// use sqlx_vrchat_sqlite_test::zaphkiel::world_instance::WorldInstance;
///
/// let world_instance = WorldInstance::new();
///
/// assert_eq!(world_instance, WorldInstance::default())
/// ```
///
/// # Creating a new `WorldInstance` using &str
/// ```
/// use sqlx_vrchat_sqlite_test::zaphkiel::world_instance::WorldInstance;
///
/// let world_instance: WorldInstance = "wrld_1234:1234~private(usr_1234)".parse().unwrap();
///
/// assert_eq!(world_instance.world_id, "wrld_1234");
/// assert_eq!(world_instance.instance_id, "1234");
/// assert_eq!(world_instance.private, Some("usr_1234".to_string()));
/// ```
///
/// # Creating a new `WorldInstance` using String
/// ```
/// use sqlx_vrchat_sqlite_test::zaphkiel::world_instance::WorldInstance;
///
/// let world_instance_string = "wrld_1234:1234~private(usr_1234)".to_string();
///
/// let world_instance: WorldInstance = WorldInstance::from(world_instance_string);
///
/// assert_eq!(world_instance.world_id, "wrld_1234");
/// assert_eq!(world_instance.instance_id, "1234");
/// assert_eq!(world_instance.private, Some("usr_1234".to_string()));
/// ```
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
    /// Create a new `WorldInstance` with default values as specified in the `Default` trait.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
}

/// A struct representing a world instance parse error.
///
/// Valid parse errors:
///
/// - `Empty`: The string is empty.
/// - `InvalidFormat`: The string is not in the correct format.
/// - `InvalidWorldId`: The world id is invalid.
/// - `InvalidInstanceId`: The instance id is invalid.
/// - `InvalidOptionalField`: The optional field is invalid.
/// - `Other`: Other errors.
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

    /// Parse a `WorldInstance` from a string.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::str::FromStr;
    /// use sqlx_vrchat_sqlite_test::zaphkiel::world_instance::WorldInstance;
    ///
    /// let world_instance = WorldInstance::from_str("wrld_1234:1234~private(usr_1234)").unwrap();
    /// assert_eq!(world_instance.world_id, "wrld_1234");
    /// assert_eq!(world_instance.instance_id, "1234");
    /// assert_eq!(world_instance.private, Some("usr_1234".to_string()));
    /// ```
    ///
    /// World instance format:
    ///
    /// ```text
    /// <world_id>:<instance_id>~<optional_field>(<optional_field_value>)
    /// ```
    ///
    /// Valid optional fields:
    ///
    /// - `nonce`: The optional nonce of the world instance.
    /// - `hidden`: The optional hidden of the world instance.
    /// - `private`: The optional private of the world instance.
    /// - `region`: The optional region of the world instance.
    ///
    /// # Errors
    ///
    /// - `Empty`: The string is empty.
    /// - `InvalidFormat`: The string is not in the correct format.
    /// - `InvalidWorldId`: The world id is invalid.
    /// - `InvalidInstanceId`: The instance id is invalid.
    /// - `InvalidOptionalField`: The optional field is invalid.
    /// - `Other`: Other errors.
    ///
    /// # Working
    ///
    /// 1. Split the string by `:`.
    /// 2. Check if the length of the split is 2.
    /// 3. Check if the first part is empty.
    /// 4. Set the world id to the first part.
    /// 5. Split the second part by `~`.
    /// 6. Check if the length of the split is 2.
    /// 7. Check if the first part is empty.
    /// 8. Set the instance id to the first part.
    /// 9. Split the second part by `(`.
    /// 10. Check if the length of the split is 2.
    /// 11. Check if the first part is empty.
    /// 12. Check if the first part is a valid optional field.
    /// 13. Set the optional field to the first part.
    /// 14. Check if the second part ends with `)`.
    /// 15. Check if the second part is empty.
    /// 16. Set the optional field value to the second part.
    ///
    /// # Notes
    ///     
    /// - The optional field value is not checked for validity.
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
    /// Parse a `WorldInstance` from a string.
    /// See `FromStr` for more information.
    /// # Examples
    /// ```
    /// use std::str::FromStr;
    /// use sqlx_vrchat_sqlite_test::zaphkiel::world_instance::WorldInstance;
    ///
    /// let world_instance = WorldInstance::from("wrld_1234:1234~private(usr_1234)");
    /// assert_eq!(world_instance.world_id, "wrld_1234");
    /// assert_eq!(world_instance.instance_id, "1234");
    /// assert_eq!(world_instance.private, Some("usr_1234".to_string()));
    /// ```
    fn from(s: &str) -> Self {
        Self::from_str(s).unwrap()
    }
}

impl From<String> for WorldInstance {
    /// Parse a `WorldInstance` from a string.
    /// See `FromStr` for more information.
    /// # Examples
    /// ```
    /// use std::str::FromStr;
    /// use sqlx_vrchat_sqlite_test::zaphkiel::world_instance::WorldInstance;
    ///
    /// let world_instance = WorldInstance::from("wrld_1234:1234~private(usr_1234)".to_string());
    /// assert_eq!(world_instance.world_id, "wrld_1234");
    /// assert_eq!(world_instance.instance_id, "1234");
    /// assert_eq!(world_instance.private, Some("usr_1234".to_string()));
    /// ```
    fn from(s: String) -> Self {
        Self::from_str(&s).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::zaphkiel::world_instance::{WorldInstance, WorldInstanceParseError};
    use crate::zaphkiel::world_regions::Regions;
    use std::str::FromStr;

    #[test]
    fn test_parse_world_instance() {
        let world_instance_str = "world_id:instance_id~region(EU)";
        let expected_world_instance = WorldInstance {
            world_id: "world_id".to_string(),
            instance_id: "instance_id".to_string(),
            nonce: None,
            hidden: None,
            private: None,
            region: Some(Regions::Europe),
            friends: None,
            group: None,
        };
        let actual_world_instance = WorldInstance::from_str(world_instance_str).unwrap();
        assert_eq!(actual_world_instance, expected_world_instance);
    }

    #[test]
    fn test_parse_world_instance_invalid_format() {
        let world_instance_str = "invalid_format";
        let actual_result = WorldInstance::from_str(world_instance_str);
        assert!(actual_result.is_err());
        assert_eq!(
            actual_result.unwrap_err(),
            WorldInstanceParseError::InvalidFormat
        );
    }

    #[test]
    fn test_parse_world_instance_invalid_world_id() {
        let world_instance_str = ":instance_id~region(EU)";
        let actual_result = WorldInstance::from_str(world_instance_str);
        assert!(actual_result.is_err());
        assert_eq!(
            actual_result.unwrap_err(),
            WorldInstanceParseError::InvalidWorldId
        );
    }

    #[test]
    fn test_parse_world_instance_invalid_instance_id() {
        let world_instance_str = "world_id:~region(EU)";
        let actual_result = WorldInstance::from_str(world_instance_str);
        assert!(actual_result.is_err());
        assert_eq!(
            actual_result.unwrap_err(),
            WorldInstanceParseError::InvalidInstanceId
        );
    }

    #[test]
    #[should_panic]
    fn test_parse_world_instance_unknown_key() {
        let world_instance_str = "world_id:instance_id~unknown_key(value)";
        let actual_result = WorldInstance::from_str(world_instance_str);
        assert!(actual_result.is_err());
        assert_eq!(actual_result.unwrap_err(), WorldInstanceParseError::Other);
    }

    #[test]
    fn test_from_str_for_world_instance_empty_input() {
        let world_instance_str = "";
        let actual_result = WorldInstance::from_str(world_instance_str);
        assert!(actual_result.is_err());
        assert_eq!(actual_result.unwrap_err(), WorldInstanceParseError::Empty);
    }

    #[test]
    fn test_from_str_for_world_instance_from_string() {
        let world_instance_str = "world_id:instance_id~region(US)";
        let expected_world_instance = WorldInstance {
            world_id: "world_id".to_string(),
            instance_id: "instance_id".to_string(),
            nonce: None,
            hidden: None,
            private: None,
            region: Some(Regions::US),
            friends: None,
            group: None,
        };
        let actual_world_instance = WorldInstance::from(world_instance_str.to_string());
        assert_eq!(actual_world_instance, expected_world_instance);
    }

    #[test]
    fn test_from_str_for_world_instance_from_str() {
        let world_instance_str = "world_id:instance_id~region(US)";
        let expected_world_instance = WorldInstance {
            world_id: "world_id".to_string(),
            instance_id: "instance_id".to_string(),
            nonce: None,
            hidden: None,
            private: None,
            region: Some(Regions::US),
            friends: None,
            group: None,
        };
        let actual_world_instance = WorldInstance::from(world_instance_str);
        assert_eq!(actual_world_instance, expected_world_instance);
    }
}
