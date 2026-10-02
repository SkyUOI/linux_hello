use std::{ops::Not as _, path};

use crate::kernel::{self, delete, load, match_, save, view};

#[derive(Default)]
pub struct NoopKernel {
    delete: bool,
}

impl kernel::FaceKernel for NoopKernel {
    fn name(&self) -> &'static str {
        "noop"
    }

    fn load_face(
        &mut self,
        id: String,
        _image: image::DynamicImage,
    ) -> Result<load::LoadResult, load::LoadError> {
        Ok(load::LoadResult { id })
        // Err(LoadError::NoFace)
    }

    fn match_face(
        &mut self,
        _image: image::DynamicImage,
    ) -> Result<match_::MatchResult, match_::MatchError> {
        Ok(match_::MatchResult {
            score: 0f32,
            best_id: "test_id".to_string(),
        })
    }

    fn view_face_list(&mut self) -> Result<view::ViewFaceListResult, view::ViewFaceListError> {
        if self.delete.not() {
            Ok(view::ViewFaceListResult {
                faces: vec![view::FaceListItem {
                    id: "test_id".to_string(),
                    created_at: "2023-01-01 00:00:00".to_string(),
                }],
            })
        } else {
            Ok(view::ViewFaceListResult { faces: vec![] })
        }
    }

    fn delete_face(&mut self, id: String) -> Result<delete::DeleteResult, delete::DeleteError> {
        self.delete = true;
        Ok(delete::DeleteResult { id })
    }

    fn save_data(&mut self) -> Result<save::SaveResult, save::SaveError> {
        log::info!("saving data");
        Ok(save::SaveResult {
            path: path::PathBuf::from("/test"),
        })
    }
}
