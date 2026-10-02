#[derive(Debug, thiserror::Error)]
pub enum ViewFaceListError {
    #[error("internal error: {0}")]
    Internal(#[from] anyhow::Error),
}

#[derive(Debug)]
pub struct FaceListItem {
    pub id: String,
    pub created_at: String,
}

#[derive(Debug)]
pub struct ViewFaceListResult {
    pub faces: Vec<FaceListItem>,
}