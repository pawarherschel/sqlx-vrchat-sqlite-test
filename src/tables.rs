pub mod gamelog_location;
pub mod sqlite_master;
pub mod usr_friend_log_current;

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Tables {
    SqliteMaster,
    GamelogLocation,
    UsrFriendLogCurrent(String),
}
