use crate::kernel::{delete, load, match_, save, view};

#[derive(Debug, thiserror::Error)]
pub enum KernelError {
    #[error("loading face error: {0}")]
    Load(#[from] load::LoadError),
    #[error("matching face error: {0}")]
    Match(#[from] match_::MatchError),
    #[error("viewing face list error: {0}")]
    ViewFaceList(#[from] view::ViewFaceListError),
    #[error("deleting face error: {0}")]
    Delete(#[from] delete::DeleteError),
    #[error("saving data error: {0}")]
    Save(#[from] save::SaveError),
}
