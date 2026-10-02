use crate::entities::faces;
use migration::MigratorTrait;
use sea_orm::{EntityTrait, IntoActiveModel};
use std::{io, path};
use tokio::runtime;

pub struct FaceStore {
    pub temp_connection: sea_orm::DatabaseConnection,
    pub persist_path: path::PathBuf,
}

impl FaceStore {
    pub fn open(
        runtime: &runtime::Runtime,
        persist_path: path::PathBuf,
    ) -> Result<Self, FaceStoreError> {
        let url = format!("sqlite://{path}?mode=rwc", path = persist_path.display());
        let persist_database = runtime.block_on(sea_orm::Database::connect(&url))?;
        let temp_connection = runtime.block_on(sea_orm::Database::connect("sqlite::memory:"))?;

        runtime.block_on(migration::Migrator::up(&temp_connection, None))?;

        for face in runtime.block_on(faces::Entity::find().all(&persist_database))? {
            runtime
                .block_on(faces::Entity::insert(face.into_active_model()).exec(&temp_connection))?;
        }

        Ok(Self {
            temp_connection,
            persist_path,
        })
    }

    pub fn persist_url(&self) -> String {
        format!(
            "sqlite://{path}?mode=rwc",
            path = self.persist_path.display()
        )
    }
}

#[derive(Debug, thiserror::Error)]
pub enum FaceStoreError {
    #[error("io error: {0}")]
    Io(#[from] io::Error),
    #[error("database error: {0}")]
    Database(#[from] sea_orm::DbErr),
}
