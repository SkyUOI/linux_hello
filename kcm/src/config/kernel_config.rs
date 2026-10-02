#[derive(serde::Deserialize, Debug, serde::Serialize)]
#[serde(default)]
pub struct KernelConfig {
    pub match_rate: f32,
}

impl Default for KernelConfig {
    fn default() -> Self {
        Self { match_rate: 0.4 }
    }
}
