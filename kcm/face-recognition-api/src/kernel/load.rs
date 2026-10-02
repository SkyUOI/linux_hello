#[derive(Debug, thiserror::Error)]
pub enum LoadError {
    #[error("no face detected")]
    NoFace,
    #[error("face {0} already enrolled")]
    Duplicated(String),
    #[error("kernel internal error: {0}")]
    Internal(#[from] anyhow::Error),
}

#[derive(Debug)]
pub struct LoadResult {
    pub id: String,
}