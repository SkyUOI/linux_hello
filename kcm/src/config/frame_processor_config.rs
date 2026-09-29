#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct FrameProcessorConfig {
    #[serde(alias = "interval-ms")]
    pub interval_ms: u32,
}

impl Default for FrameProcessorConfig {
    fn default() -> Self {
        Self { interval_ms: 100 }
    }
}
