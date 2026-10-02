use std::io;

pub mod delete;
pub mod load;
pub mod match_;
pub mod save;
pub mod view;

pub trait FaceKernel: Send {
    fn name(&self) -> &'static str;

    fn load_face(
        &mut self,
        id: String,
        image: image::DynamicImage,
    ) -> Result<load::LoadResult, load::LoadError>;

    fn match_face(
        &mut self,
        image: image::DynamicImage,
    ) -> Result<match_::MatchResult, match_::MatchError>;

    fn view_face_list(&mut self) -> Result<view::ViewFaceListResult, view::ViewFaceListError>;

    fn delete_face(&mut self, id: String) -> Result<delete::DeleteResult, delete::DeleteError>;

    fn save_data(&mut self) -> Result<save::SaveResult, save::SaveError>;
}

#[derive(Debug, thiserror::Error)]
pub enum LaunchError {
    #[error("data path not found")]
    DataPathNotFound,
    #[error("io error: {0}")]
    Io(#[from] io::Error),
    #[error("kernel internal error: {0}")]
    Internal(#[from] anyhow::Error),
}
