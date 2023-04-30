pub mod models {
    pub mod app_config;
    pub mod connection;
    pub mod database;
    pub mod gamelog_join_leave;
    pub mod gamelog_location;
    pub mod usr_friend_log_current;
}

pub mod rows {
    pub mod gamelog_join_leave;
    pub mod gamelog_location;
    pub mod sqlite_master;
    pub mod usr_friend_log_current;
}

pub mod zaphkiel {
    pub mod join_leave_event;
    pub mod trust_level;
    pub mod world_instance;
    pub mod world_regions;
}
