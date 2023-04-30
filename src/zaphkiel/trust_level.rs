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
