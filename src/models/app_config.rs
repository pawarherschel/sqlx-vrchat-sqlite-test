use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;

use config::Config;
use sqlx::migrate::MigrateDatabase;
use sqlx::Sqlite;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct AppConfig {
    pub vrcx_sqlite: PathBuf,
    pub url: Option<String>,
    pub usr_id: String,
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
            usr_id: String::new(),
            url: None,
            verbose: false,
        }
    }
}

impl AppConfig {
    fn new() -> Self {
        AppConfig::default()
    }
    pub async fn build<'a>(self) -> Result<AppConfig, Box<dyn Error>> {
        if !self.vrcx_sqlite.exists() {
            panic!("The sqlite3 file does not exist at {:?}", self.vrcx_sqlite);
        }
        let path_str = self.vrcx_sqlite.to_str();
        let path_url = if let Some(path_url) = &path_str {
            if !Sqlite::database_exists(path_url).await? {
                panic!(
                    "Database file exists but the database does not exist at {:?}",
                    self.vrcx_sqlite
                );
            } else {
                println!("Database exists at {:?}", self.vrcx_sqlite);
            }

            path_url
        } else {
            panic!("Error converting {path:?} to url", path = self.vrcx_sqlite);
        };

        Ok(AppConfig {
            vrcx_sqlite: self.vrcx_sqlite.clone(),
            url: Some(path_url.to_string()),
            usr_id: self.usr_id,
            verbose: self.verbose,
        })
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
                "usr_id" => config.usr_id = value,
                _ => {}
            }
        }

        config
    }
}
