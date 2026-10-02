#[derive(Debug, thiserror::Error)]
pub enum MatchError {
    #[error("no face detected")]
    NoFace,
    #[error("kernel internal error: {0}")]
    Internal(#[from] anyhow::Error),
    #[error("no face enrolled")]
    NoEnrolled,
}

#[derive(Debug)]
pub struct MatchResult {
    pub score: f32,
    pub best_id: String,
}
