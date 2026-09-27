use std::path;

#[derive(serde::Deserialize, Debug, serde::Serialize)]
#[serde(default)]
pub struct CameraConfig {
    #[serde(rename = "system-path")]
    pub camera_system_path: Option<path::PathBuf>,
    pub mirrored: bool,
    pub running: bool,
}

impl Default for CameraConfig {
    fn default() -> Self {
        Self {
            camera_system_path: Default::default(),
            mirrored: Default::default(),
            running: true,
        }
    }
}
