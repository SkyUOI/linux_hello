use crate::{common, entities::faces, store};
use anyhow::Context as _;
use face_id::{analyzer, error};
use face_recognition_api::kernel::{self, delete, load, match_, save, view};
use jiff_chrono_conversions::TryToJiff;
use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait, IntoActiveModel, PaginatorTrait};
use tokio::runtime;

pub struct FaceIdKernel {
    analyzer: analyzer::FaceAnalyzer,
    store: store::FaceStore,
    runtime: runtime::Runtime,
}

impl FaceIdKernel {
    pub fn new() -> Result<Self, kernel::LaunchError> {
        let Some(data_path) = common::get_data_path() else {
            log::error!("kernel launches failed: cannot get data path");
            return Err(kernel::LaunchError::DataPathNotFound);
        };
        let models_path = data_path.join("models").join("face-id-kernel");
        let runtime = runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;
        log::debug!("models path: {:?}", data_path);
        let analyzer = analyzer::FaceAnalyzer::builder(
            &models_path.join("2.5g_bnkps.onnx"),
            &models_path.join("w600k_mbf.onnx"),
            &models_path.join("genderage.onnx"),
        )
        .build()
        .context("cannot launch face-id kernel")?;
        let store = store::FaceStore::open(
            &runtime,
            data_path
                .join("database")
                .join("face-id-kernel")
                .join("data.db"),
        )
        .map_err(anyhow::Error::msg)?;
        Ok(Self {
            analyzer,
            store,
            runtime,
        })
    }

    fn describe(&self, image: &image::DynamicImage) -> Result<Vec<f32>, FaceIdKernelError> {
        let analyzed_faces = self.analyzer.analyze(image)?;
        let Some(embedding) = analyzed_faces
            .iter()
            .max_by(|lft, rgt| lft.detection.score.total_cmp(&rgt.detection.score))
            .map(|face| face.embedding.clone())
        else {
            return Err(FaceIdKernelError::NoFaceDetected);
        };

        Ok(embedding)
    }
}

#[derive(Debug, thiserror::Error)]
enum FaceIdKernelError {
    #[error("face-id error: {0}")]
    FaceId(#[from] error::FaceIdError),
    #[error("no face detected")]
    NoFaceDetected,
}

impl kernel::FaceKernel for FaceIdKernel {
    fn name(&self) -> &'static str {
        "face-id kernel"
    }

    fn load_face(
        &mut self,
        id: String,
        image: image::DynamicImage,
    ) -> Result<load::LoadResult, load::LoadError> {
        match self
            .runtime
            .block_on(faces::Entity::find_by_id(&id).one(&self.store.temp_connection))
        {
            Ok(Some(_)) => return Err(load::LoadError::Duplicated(id)),
            Ok(None) => {}
            Err(e) => return Err(anyhow::anyhow!("{e}").into()),
        }

        let embedding = match self.describe(&image) {
            Ok(embedding) => embedding,
            Err(FaceIdKernelError::NoFaceDetected) => return Err(load::LoadError::NoFace),
            Err(FaceIdKernelError::FaceId(e)) => return Err(anyhow::anyhow!("{e}").into()),
        };

        let norm = crate::maths::norm(&embedding);

        let embedding: Vec<_> = embedding.iter().flat_map(|x| x.to_le_bytes()).collect();

        let face = faces::ActiveModel {
            id: ActiveValue::Set(id.clone()),
            embedding: ActiveValue::Set(embedding),
            norm: ActiveValue::Set(norm),
            ..Default::default()
        };

        self.runtime
            .block_on(face.insert(&self.store.temp_connection))
            .map_err(anyhow::Error::msg)?;

        Ok(load::LoadResult { id })
    }

    fn match_face(
        &mut self,
        image: image::DynamicImage,
    ) -> Result<match_::MatchResult, match_::MatchError> {
        if self
            .runtime
            .block_on(faces::Entity::find().count(&self.store.temp_connection))
            .map_err(anyhow::Error::msg)?
            == 0
        {
            return Err(match_::MatchError::NoEnrolled);
        }

        let embedding = match self.describe(&image) {
            Ok(embedding) => embedding,
            Err(FaceIdKernelError::NoFaceDetected) => return Err(match_::MatchError::NoFace),
            Err(FaceIdKernelError::FaceId(e)) => return Err(anyhow::anyhow!("{e}").into()),
        };
        let norm = crate::maths::norm(&embedding);

        let faces = self
            .runtime
            .block_on(faces::Entity::find().all(&self.store.temp_connection))
            .map_err(anyhow::Error::msg)?;
        let (best_id, score) = faces
            .iter()
            .map(|face| {
                (
                    &face.id,
                    face.embedding
                        .as_chunks::<4>()
                        .0
                        .iter()
                        .map(|&bytes| f32::from_le_bytes(bytes))
                        .zip(embedding.iter())
                        .map(|(x, y)| x * y)
                        .sum::<f32>()
                        / face.norm
                        / norm,
                )
            })
            .max_by(|lft, rgt| lft.1.total_cmp(&rgt.1))
            .map(|(id, score)| (id.clone(), score))
            .ok_or(match_::MatchError::NoEnrolled)?;

        Ok(match_::MatchResult { score, best_id })
    }

    fn view_face_list(&mut self) -> Result<view::ViewFaceListResult, view::ViewFaceListError> {
        Ok(view::ViewFaceListResult {
            faces: self
                .runtime
                .block_on(faces::Entity::find().all(&self.store.temp_connection))
                .map_err(anyhow::Error::msg)?
                .iter()
                .map(
                    |model| -> Result<view::FaceListItem, view::ViewFaceListError> {
                        Ok(view::FaceListItem {
                            id: model.id.clone(),
                            created_at: model
                                .created_at
                                .try_to_jiff()
                                .map_err(anyhow::Error::msg)?
                                .strftime("%Y-%m-%d %H:%M:%S")
                                .to_string(),
                        })
                    },
                )
                .collect::<Result<Vec<_>, _>>()?,
        })
    }

    fn delete_face(&mut self, id: String) -> Result<delete::DeleteResult, delete::DeleteError> {
        self.runtime
            .block_on(faces::Entity::delete_by_id(&id).exec(&self.store.temp_connection))
            .map_err(anyhow::Error::msg)?;
        Ok(delete::DeleteResult { id })
    }

    fn save_data(&mut self) -> Result<save::SaveResult, save::SaveError> {
        let persist_database = self
            .runtime
            .block_on(sea_orm::Database::connect(&self.store.persist_url()))
            .map_err(anyhow::Error::msg)?;
        let add_faces = self
            .runtime
            .block_on(faces::Entity::find().all(&self.store.temp_connection))
            .map_err(anyhow::Error::msg)?
            .iter()
            .filter_map(|model| {
                match self
                    .runtime
                    .block_on(faces::Entity::find_by_id(&model.id).one(&persist_database))
                {
                    Ok(Some(_)) => None,
                    Ok(None) => Some(Ok(model.clone().into_active_model())),
                    Err(e) => Some(Err(save::SaveError::from(anyhow::anyhow!("{e}")))),
                }
            })
            .collect::<Result<Vec<_>, _>>()?;
        self.runtime
            .block_on(faces::Entity::find().all(&persist_database))
            .map_err(anyhow::Error::msg)?
            .iter()
            .filter_map(|model| {
                match self
                    .runtime
                    .block_on(faces::Entity::find_by_id(&model.id).one(&self.store.temp_connection))
                {
                    Ok(Some(_)) => None,
                    Ok(None) => {
                        Some(self.runtime.block_on(
                            faces::Entity::delete_by_id(&model.id).exec(&persist_database),
                        ))
                    }
                    Err(e) => Some(Err(e)),
                }
            })
            .collect::<Result<Vec<_>, _>>()
            .map_err(anyhow::Error::msg)?;
        self.runtime
            .block_on(faces::Entity::insert_many(add_faces).exec(&persist_database))
            .map_err(anyhow::Error::msg)?;
        Ok(save::SaveResult {
            path: self.store.persist_path.clone(),
        })
    }
}
