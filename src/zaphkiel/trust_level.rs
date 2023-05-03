/// Trust level of a user.
///
/// # Trust Levels
/// - Unknown
/// - Visitor
/// - New User
/// - User
/// - Known User
/// - Trusted User
/// - VRChat Team
/// - Nuisance
///
/// # Examples
/// ```
/// use sqlx_vrchat_sqlite_test::zaphkiel::trust_level::TrustLevel;
///
/// let trust_level = TrustLevel::from("user");
/// assert_eq!(trust_level, TrustLevel::User);
///     
/// let trust_level = TrustLevel::from("User");
/// assert_eq!(trust_level, TrustLevel::User);
///
/// let trust_level = TrustLevel::from("USER");
/// assert_eq!(trust_level, TrustLevel::User);
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
pub enum TrustLevel {
    #[default]
    Unknown,
    Visitor,
    NewUser,
    User,
    KnownUser,
    TrustedUser,
    VRChatTeam,
    Nuisance,
}

impl From<&str> for TrustLevel {
    fn from(value: &str) -> Self {
        let value = value.to_lowercase();
        Self::from(value)
    }
}

impl From<String> for TrustLevel {
    fn from(value: String) -> Self {
        let value = value.to_lowercase();
        match value.as_str() {
            "visitor" => TrustLevel::Visitor,
            "new user" => TrustLevel::NewUser,
            "user" => TrustLevel::User,
            "known user" => TrustLevel::KnownUser,
            "trusted user" => TrustLevel::TrustedUser,
            "vrchat team" => TrustLevel::VRChatTeam,
            "nuisance" => TrustLevel::Nuisance,

            "new_user" => TrustLevel::NewUser,
            "known_user" => TrustLevel::KnownUser,
            "trusted_user" => TrustLevel::TrustedUser,
            "vrchat_team" => TrustLevel::VRChatTeam,

            _ => panic!("Unknown trust level: {}", value),
        }
    }
}
