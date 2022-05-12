use std::path::PathBuf;

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Copy, Debug, Default)]
pub struct Config {
    pub is_fullscreen: bool,
}

use crate::errors::ConfigError;

lazy_static! {
    static ref PROJECT_DIRS: ProjectDirs = ProjectDirs::from("", "jewelexx", "herdes").unwrap();
    static ref CONFIG_DIR: PathBuf = PROJECT_DIRS.config_dir().to_path_buf();
    static ref CONFIG_PATH: PathBuf = (*CONFIG_DIR).join("config.json");
}

impl Config {
    pub fn init() -> Result<Self, ConfigError> {
        let config_path = CONFIG_PATH.to_owned();

        if !(CONFIG_DIR.to_owned()).exists() {
            std::fs::create_dir_all(CONFIG_DIR.to_owned())?;
        }

        let config = {
            if config_path.exists() {
                let file = std::fs::File::open(&config_path)?;
                let reader = std::io::BufReader::new(file);
                serde_json::from_reader(reader)?
            } else {
                let file = std::fs::File::create(&config_path)?;
                let writer = std::io::BufWriter::new(file);
                serde_json::to_writer(writer, &Config::default())?;

                Config::default()
            }
        };

        Ok(config)
    }
}
