use std::str::FromStr;

/// JoinLeaveEvent is an enum that represents the different types of join/leave events that can be
/// found in the gamelog.
///
/// # Available Variants
/// - Join
/// - Leave
/// - Other
///
/// # Examples
///
/// ```
/// use sqlx_vrchat_sqlite_test::zaphkiel::join_leave_event::JoinLeaveEvent;
///
/// let join = JoinLeaveEvent::from("OnPlayerJoined");
/// let leave = JoinLeaveEvent::from("OnPlayerLeft");
///
/// assert_eq!(join, JoinLeaveEvent::Join);
/// assert_eq!(leave, JoinLeaveEvent::Leave);
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
pub enum JoinLeaveEvent {
    Join,
    Leave,
    #[default]
    Other,
}

impl From<&str> for JoinLeaveEvent {
    fn from(value: &str) -> Self {
        let value = value.to_lowercase();
        Self::from(value)
    }
}

impl From<String> for JoinLeaveEvent {
    fn from(value: String) -> Self {
        let value = value.to_lowercase();
        match value.as_str() {
            "join" => JoinLeaveEvent::Join,
            "leave" => JoinLeaveEvent::Leave,

            "joins" => JoinLeaveEvent::Join,
            "leaves" => JoinLeaveEvent::Leave,

            "joined" => JoinLeaveEvent::Join,
            "left" => JoinLeaveEvent::Leave,

            "onplayerjoined" => JoinLeaveEvent::Join,
            "onplayerleft" => JoinLeaveEvent::Leave,

            _ => panic!("Unknown join/leave event: {}", value),
        }
    }
}

impl FromStr for JoinLeaveEvent {
    type Err = std::string::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s))
    }
}
