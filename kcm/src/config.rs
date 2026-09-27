use std::{
    fs,
    io::{self, Read},
    ops::Not as _,
    path,
};

pub mod camera_config;
pub mod log_manager_config;

/// The configuration of Kcm.
///
/// The type of configuration is `TOML`. If the configuration
/// cannot be found or any key unset, it will be default.
#[derive(Default, serde::Deserialize, Debug, serde::Serialize)]
#[serde(default)]
pub struct Config {
    #[serde(rename = "camera")]
    pub camera_config: camera_config::CameraConfig,
    #[serde(rename = "log")]
    pub log_manager_config: log_manager_config::LogManagerConfig,
}

impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        let config_path = Self::get_config_path();
        if let Some(config_path) = config_path {
            if config_path.exists().not() {
                Ok(Self::default())
            } else {
                let mut config_file = fs::OpenOptions::new().read(true).open(config_path)?;
                let mut content = String::new();
                config_file.read_to_string(&mut content)?;
                Ok(toml::from_str::<Self>(&content)?)
            }
        } else {
            Ok(Self::default())
        }
    }

    pub fn get_config_path() -> Option<path::PathBuf> {
        let config_parent_path = dirs::config_dir();
        config_parent_path.map(|dir| dir.join("linux-hello").join("config.toml"))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ConfigError {
    #[error("parsing configuration from TOML error")]
    DeserializeParse(#[from] toml::de::Error),
    #[error("parsing configuration to TOML error")]
    SerializeParse(#[from] toml::ser::Error),
    #[error("filesystem io error")]
    Io(#[from] io::Error),
    #[error("getting config path error")]
    GetConfigPath,
}
