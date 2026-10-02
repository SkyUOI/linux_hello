#[derive(serde::Deserialize, Debug, serde::Serialize)]
#[serde(default)]
pub struct KernelConfig {
    pub match_score: f32,
}

impl Default for KernelConfig {
    fn default() -> Self {
        Self { match_score: 0.4 }
    }
}
