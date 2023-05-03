use std::str::FromStr;

/// Regions where VRChat worlds instances are hosted.
///
/// # Available Regions
/// - USWest
/// - US
/// - USEast
/// - Europe
/// - Japan
/// - Other
///
/// # Examples
/// ```
/// use sqlx_vrchat_sqlite_test::zaphkiel::world_regions::Regions;
///
/// let region = Regions::from("uswest");
/// assert_eq!(region, Regions::USWest);
/// ```
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
pub enum Regions {
    #[default]
    Other,
    USWest,
    US,
    USEast,
    Europe,
    Japan,
}

impl From<&str> for Regions {
    fn from(value: &str) -> Self {
        let value = value.to_lowercase();
        Self::from(value)
    }
}

impl From<String> for Regions {
    fn from(value: String) -> Self {
        let value = value.to_lowercase();
        match value.as_str() {
            "uswest" => Regions::USWest,
            "us" => Regions::US,
            "useast" => Regions::USEast,
            "europe" => Regions::Europe,
            "japan" => Regions::Japan,

            "usw" => Regions::USWest,
            "use" => Regions::USEast,
            "eu" => Regions::Europe,
            "jp" => Regions::Japan,

            "us w" => Regions::USWest,
            "us e" => Regions::USEast,

            "us_w" => Regions::USWest,
            "us_e" => Regions::USEast,

            "uw" => Regions::USWest,
            "ue" => Regions::USEast,

            _ => panic!("Unknown region: {}", value),
        }
    }
}

impl FromStr for Regions {
    type Err = std::string::ParseError;

    fn from_str(s: &str) -> Result<Regions, Self::Err> {
        Ok(Self::from(s))
    }
}
