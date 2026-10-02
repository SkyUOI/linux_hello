use crate::kernel::{delete, load, match_, save, view};

#[derive(Debug)]
pub enum KernelResult {
    Match(match_::MatchResult),
    Load(load::LoadResult),
    ViewFaceList(view::ViewFaceListResult),
    Delete(delete::DeleteResult),
    Save(save::SaveResult),
}

impl From<match_::MatchResult> for KernelResult {
    fn from(value: match_::MatchResult) -> Self {
        Self::Match(value)
    }
}

impl From<load::LoadResult> for KernelResult {
    fn from(value: load::LoadResult) -> Self {
        Self::Load(value)
    }
}

impl From<view::ViewFaceListResult> for KernelResult {
    fn from(value: view::ViewFaceListResult) -> Self {
        Self::ViewFaceList(value)
    }
}

impl From<delete::DeleteResult> for KernelResult {
    fn from(value: delete::DeleteResult) -> Self {
        Self::Delete(value)
    }
}

impl From<save::SaveResult> for KernelResult {
    fn from(value: save::SaveResult) -> Self {
        Self::Save(value)
    }
}