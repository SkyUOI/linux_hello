use std::{path, str};

#[derive(serde::Deserialize, Debug, serde::Serialize)]
#[serde(default)]
pub struct LogManagerConfig {
    /// env var is `KCM_LOG_LEVEL`
    #[serde(rename = "level")]
    pub log_level_filter: LevelFilter,
    /// env var is `KCM_LOG_PATH`
    #[serde(rename = "path")]
    pub log_file_path: Option<path::PathBuf>,
}

#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
pub enum LevelFilter {
    Off,
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl Default for LogManagerConfig {
    fn default() -> Self {
        Self {
            log_level_filter: LevelFilter::Debug,
            log_file_path: Default::default(),
        }
    }
}

impl From<LevelFilter> for log::LevelFilter {
    fn from(value: LevelFilter) -> Self {
        match value {
            LevelFilter::Off => log::LevelFilter::Off,
            LevelFilter::Trace => log::LevelFilter::Trace,
            LevelFilter::Debug => log::LevelFilter::Debug,
            LevelFilter::Info => log::LevelFilter::Info,
            LevelFilter::Warn => log::LevelFilter::Warn,
            LevelFilter::Error => log::LevelFilter::Error,
        }
    }
}

impl str::FromStr for LevelFilter {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "off" => Ok(Self::Off),
            "trace" => Ok(Self::Trace),
            "debug" => Ok(Self::Debug),
            "info" => Ok(Self::Info),
            "warn" => Ok(Self::Warn),
            "error" => Ok(Self::Error),
            _ => Err("invalid value"),
        }
    }
}