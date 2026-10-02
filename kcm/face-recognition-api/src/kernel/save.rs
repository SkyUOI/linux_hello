use std::path;

#[derive(Debug)]
pub struct SaveResult {
    pub path: path::PathBuf,
}

#[derive(Debug, thiserror::Error)]
pub enum SaveError {
    #[error("internal error: {0}")]
    Internal(#[from] anyhow::Error),
}
