use config::Config;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub vrcx_sqlite: PathBuf,
    pub verbose: bool,
}

impl AppConfig {
    fn default() -> AppConfig {
        AppConfig {
            vrcx_sqlite: directories::BaseDirs::new()
                .unwrap()
                .home_dir()
                .join("APPDATA")
                .join("VRCX")
                .join("vrcx.sqlite"),
            verbose: false,
        }
    }
}

impl AppConfig {
    fn new() -> Self {
        AppConfig::default()
    }

    pub fn get() -> Self {
        let settings = Config::builder()
            .add_source(config::File::with_name("src/Settings.toml"))
            .build()
            .unwrap();

        let settings: AppConfig = settings
            .try_deserialize::<HashMap<String, String>>()
            .unwrap()
            .into();

        settings
    }
}

impl From<HashMap<String, String>> for AppConfig {
    fn from(map: HashMap<String, String>) -> Self {
        let mut config = AppConfig::new();

        for (key, value) in map {
            match key.as_str() {
                "vrcx_sqlite" => config.vrcx_sqlite = PathBuf::from(value),
                "verbose" => config.verbose = value.parse().unwrap_or_default(),
                _ => {}
            }
        }

        config
    }
}

#[derive(Debug)]
pub struct GamelogLocation {
    pub id: i64,
    pub created_at: String,
    pub location: String,
    pub world_id: String,
    pub world_name: String,
    pub time: Option<i64>,
    pub group_name: Option<String>,
}

#[derive(Debug, Default)]
pub struct MasterTable {
    pub type_: String,
    pub name: String,
    pub tbl_name: String,
    pub rootpage: i32,
    pub sql: String,
}
