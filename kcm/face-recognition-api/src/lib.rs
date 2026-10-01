#[derive(Clone)]
pub struct FaceIdImage {
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub data: Vec<u8>,
}

pub trait FaceKernel: Send {
    fn name(&self) -> &'static str;

    fn load_face(&mut self, id: String, image: FaceIdImage) -> Result<LoadResult, LoadError>;

    fn match_face(&mut self, image: FaceIdImage) -> Result<MatchResult, MatchError>;

    fn view_face_list(&mut self) -> Result<ViewFaceListResult, ViewFaceListError>;
}

#[derive(Debug, thiserror::Error)]
pub enum KernelError {
    #[error("loading face error: {0}")]
    Load(#[from] LoadError),
    #[error("matching face error: {0}")]
    Match(#[from] MatchError),
    #[error("viewing face list error: {0}")]
    ViewFaceList(#[from] ViewFaceListError),
}

pub struct LoadResult {
    pub id: String,
}

#[derive(Debug, thiserror::Error)]
pub enum LoadError {
    #[error("no face detected")]
    NoFace,
    #[error("face {0} already enrolled")]
    Duplicated(String),
    #[error("kernel internal error")]
    Internal(#[from] anyhow::Error),
}

pub struct FaceListItem {
    pub id: String,
    pub created_at: String,
}

pub struct ViewFaceListResult {
    pub faces: Vec<FaceListItem>,
}

#[derive(Debug, thiserror::Error)]
pub enum ViewFaceListError {
    #[error("kernel internal error")]
    Internal(#[from] anyhow::Error),
}

pub struct MatchResult {
    pub score: f32,
    pub best_id: String,
}

#[derive(Debug, thiserror::Error)]
pub enum MatchError {
    #[error("no face detected")]
    NoFace,
    #[error("kernel internal error")]
    Internal(#[from] anyhow::Error),
}

pub enum KernelResult {
    Match(MatchResult),
    Load(LoadResult),
    ViewFaceList(ViewFaceListResult),
}

impl From<MatchResult> for KernelResult {
    fn from(value: MatchResult) -> Self {
        Self::Match(value)
    }
}

impl From<LoadResult> for KernelResult {
    fn from(value: LoadResult) -> Self {
        Self::Load(value)
    }
}

impl From<ViewFaceListResult> for KernelResult {
    fn from(value: ViewFaceListResult) -> Self {
        Self::ViewFaceList(value)
    }
}

#[derive(Default)]
pub struct NoopKernel;

impl FaceKernel for NoopKernel {
    fn name(&self) -> &'static str {
        "noop"
    }

    fn load_face(&mut self, id: String, _image: FaceIdImage) -> Result<LoadResult, LoadError> {
        Ok(LoadResult { id })
        // Err(LoadError::NoFace)
    }

    fn match_face(&mut self, _image: FaceIdImage) -> Result<MatchResult, MatchError> {
        Ok(MatchResult {
            score: 0f32,
            best_id: "test_id".to_string(),
        })
    }

    fn view_face_list(&mut self) -> Result<ViewFaceListResult, ViewFaceListError> {
        Ok(ViewFaceListResult {
            faces: vec![FaceListItem {
                id: "test_id".to_string(),
                created_at: "2023-01-01 00:00:00".to_string(),
            }],
        })
    }
}
