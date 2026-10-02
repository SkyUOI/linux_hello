#[derive(Debug, thiserror::Error)]
pub enum DeleteError {
    #[error("face {0} is not enrolled")]
    NotEnrolled(String),
    #[error("kernel internal error")]
    Internal(#[from] anyhow::Error),
}

#[derive(Debug)]
pub struct DeleteResult {
    pub id: String,
}