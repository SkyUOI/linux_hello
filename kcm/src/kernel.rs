use crate::{
    config::{self},
    message_manager, qobject,
};
use cxx_qt::{CxxQtType, Threading};
use face_recognition_api::{
    error,
    kernel::{self, load, match_, save},
    result,
};
use std::{cell, panic, pin, rc, sync};

pub type SharedKernel = sync::Arc<sync::Mutex<dyn kernel::FaceKernel + Send>>;

pub fn create_kernel() -> Result<SharedKernel, kernel::LaunchError> {
    #[cfg(feature = "noop-kernel")]
    {
        use face_recognition_api::noop;

        Ok(sync::Arc::new(
            sync::Mutex::new(noop::NoopKernel::default()),
        ))
    }
    #[cfg(feature = "face-id-kernel")]
    {
        use face_id_kernel::kernel;

        Ok(sync::Arc::new(sync::Mutex::new(
            kernel::FaceIdKernel::new()?
        )))
    }
}

#[derive(Default)]
pub struct FaceRecognitionKernel {
    kernel: Option<SharedKernel>,
    last_frame: sync::Arc<sync::Mutex<Option<image::DynamicImage>>>,
    pub busy: bool,
    result: Option<Result<result::KernelResult, error::KernelError>>,
    match_score: f32,
    config: Option<rc::Rc<cell::RefCell<config::Config>>>,
}

impl qobject::FaceRecognitionKernel {
    /// # Safety
    ///
    /// Only run this function after kcm is constructed.
    pub unsafe fn init(
        mut self: pin::Pin<&mut Self>,
        frame_capturer: *mut qobject::FrameCapturer,
        kcm: *mut qobject::Kcm,
    ) {
        if kcm.is_null() {
            log::error!("kcm is null");
            return;
        }
        if frame_capturer.is_null() {
            log::error!("frame capturer is null");
            return;
        }
        let config = rc::Rc::clone(&unsafe { &*kcm }.config);
        self.as_mut().rust_mut().match_score = config.borrow().kernel_config.match_score;
        self.as_mut().rust_mut().config = Some(config);
        let kernel = match create_kernel() {
            Ok(kernel) => kernel,
            Err(e) => {
                log::error!("create face recognition kernel failed: {e}");
                return;
            }
        };
        self.as_mut().rust_mut().kernel = Some(kernel);
        self.as_mut().rust_mut().last_frame = unsafe { &*frame_capturer }.last_frame.clone();
        log::info!("face recognition kernel has been loaded");
    }

    fn get_kernel_with_last_frame(
        self: pin::Pin<&mut Self>,
    ) -> Option<(SharedKernel, image::DynamicImage)> {
        let last_frame = {
            let last_frame = match self.last_frame.lock() {
                Ok(last_frame) => last_frame,
                Err(e) => {
                    log::error!("last_frame has been poisoned: {e}");
                    return None;
                }
            };
            match last_frame.as_ref().map(Clone::clone) {
                Some(last_frame) => last_frame,
                None => {
                    log::warn!("last_frame has not been captured yet");
                    return None;
                }
            }
        };
        let Some(kernel) = self.kernel.as_ref().map(Clone::clone) else {
            log::error!("face recognition kernel is not constructed");
            return None;
        };
        Some((kernel, last_frame))
    }

    pub fn load_face(mut self: pin::Pin<&mut Self>, id: cxx_qt_lib::QString) {
        log::info!("start loading face with id: {id}");

        let Some((kernel, last_frame)) = self.as_mut().get_kernel_with_last_frame() else {
            log::error!("get kernel and last frame failed");
            return;
        };
        let id = String::from(id);
        self.as_mut().set_busy(true);

        let qt_thread = self.qt_thread();

        std::thread::spawn(move || {
            let result = std::panic::catch_unwind(panic::AssertUnwindSafe(move || {
                kernel
                    .lock()
                    .unwrap_or_else(|p| p.into_inner())
                    .load_face(id, last_frame)
            }));

            if let Err(e) = qt_thread.queue(move |mut face_recognition_kernel| {
                face_recognition_kernel.as_mut().set_busy(false);
                let result = match result {
                    Ok(result) => result.map(From::from).map_err(From::from),
                    Err(_) => Err(load::LoadError::from(anyhow::anyhow!(
                        "inner kernel might be panicked"
                    ))
                    .into()),
                };
                if let Err(e) = result.as_ref() {
                    log::error!("loading face error: {e}");
                }
                face_recognition_kernel.as_mut().rust_mut().result = Some(result);
                face_recognition_kernel.as_mut().face_loaded();
            }) {
                log::error!("threading queue error: {e}");
            }
        });

        #[cfg(feature = "test")]
        {
            use std::path;

            let width = last_frame.width;
            let height = last_frame.height;
            let stride = last_frame.stride;
            let data = &last_frame.data;
            let mut gray_image = image::GrayImage::new(width, height);

            for (i, row) in (0..height as usize).zip(gray_image.rows_mut()) {
                let row_start = i * stride as usize;
                row.zip(&data[row_start..][..width as usize])
                    .for_each(|(pixel, &data_pixel)| *pixel = [data_pixel].into());
            }

            gray_image
                .save(
                    path::Path::new("./kcm/test_image/")
                        .join(id)
                        .with_extension("png"),
                )
                .unwrap();
        }
    }

    pub fn match_face(mut self: pin::Pin<&mut Self>) {
        log::info!("start matching face");
        let Some((kernel, last_frame)) = self.as_mut().get_kernel_with_last_frame() else {
            log::error!("get kernel and last frame failed");
            return;
        };
        self.as_mut().set_busy(true);

        let qt_thread = self.qt_thread();

        std::thread::spawn(move || {
            let result = std::panic::catch_unwind(panic::AssertUnwindSafe(move || {
                kernel
                    .lock()
                    .unwrap_or_else(|p| p.into_inner())
                    .match_face(last_frame)
            }));

            if let Err(e) = qt_thread.queue(move |mut face_recognition_kernel| {
                face_recognition_kernel.as_mut().set_busy(false);
                let result = match result {
                    Ok(result) => result.map(From::from).map_err(From::from),
                    Err(_) => Err(match_::MatchError::from(anyhow::anyhow!(
                        "inner kernel might be panicked"
                    ))
                    .into()),
                };
                if let Err(e) = result.as_ref() {
                    log::error!("matching face error: {e}");
                }
                face_recognition_kernel.as_mut().rust_mut().result = Some(result);
                face_recognition_kernel.as_mut().face_matched();
            }) {
                log::error!("threading queue error: {e}");
            } else {
                log::info!("loading face successfully");
            }
        });
    }

    pub fn view_face_list(self: pin::Pin<&mut Self>) {
        log::info!("viewing face list");
        let Some(kernel) = self.kernel.as_ref().map(Clone::clone) else {
            log::error!("face recognition kernel is not constructed");
            return;
        };

        let qt_thread = self.qt_thread();

        std::thread::spawn(move || {
            let result = std::panic::catch_unwind(panic::AssertUnwindSafe(move || {
                kernel
                    .lock()
                    .unwrap_or_else(|p| p.into_inner())
                    .view_face_list()
            }));

            if let Err(e) = qt_thread.queue(move |mut face_recognition_kernel| {
                face_recognition_kernel.as_mut().set_busy(false);
                let result = match result {
                    Ok(result) => result.map(From::from).map_err(From::from),
                    Err(_) => Err(match_::MatchError::from(anyhow::anyhow!(
                        "inner kernel might be panicked"
                    ))
                    .into()),
                };
                if let Err(e) = result.as_ref() {
                    log::error!("matching face error: {e}");
                }
                face_recognition_kernel.as_mut().rust_mut().result = Some(result);
                face_recognition_kernel.as_mut().list_generated();
            }) {
                log::error!("threading queue error: {e}");
            } else {
                log::info!("viewing face successfully");
            }
        });
    }

    pub fn delete_face(mut self: pin::Pin<&mut Self>, id: cxx_qt_lib::QString) {
        log::info!("deleting face id: {}", id);

        let Some(kernel) = self.kernel.as_ref().map(Clone::clone) else {
            log::error!("face recognition kernel is not constructed");
            return;
        };

        let id = String::from(id);
        self.as_mut().set_busy(true);

        let qt_thread = self.qt_thread();

        std::thread::spawn(move || {
            let result = std::panic::catch_unwind(panic::AssertUnwindSafe(move || {
                kernel
                    .lock()
                    .unwrap_or_else(|p| p.into_inner())
                    .delete_face(id)
            }));

            if let Err(e) = qt_thread.queue(move |mut face_recognition_kernel| {
                face_recognition_kernel.as_mut().set_busy(false);
                let result = match result {
                    Ok(result) => result.map(From::from).map_err(From::from),
                    Err(_) => Err(load::LoadError::from(anyhow::anyhow!(
                        "inner kernel might be panicked"
                    ))
                    .into()),
                };
                if let Err(e) = result.as_ref() {
                    log::error!("deleting face error: {e}");
                }
                face_recognition_kernel.as_mut().rust_mut().result = Some(result);
                face_recognition_kernel.as_mut().face_deleted();
            }) {
                log::error!("threading queue error: {e}");
            } else {
                log::info!("deleting face successfully");
            }
        });
    }

    pub fn save_data(mut self: pin::Pin<&mut Self>) {
        let Some(kernel) = self.kernel.as_ref().map(Clone::clone) else {
            log::error!("face recognition kernel is not constructed");
            return;
        };
        {
            let Some(mut config) = self.config.as_ref().map(|config| config.borrow_mut()) else {
                log::error!("face recognition config is not constructed");
                return;
            };
            config.kernel_config.match_score = self.match_score;
        }
        let result = std::panic::catch_unwind(panic::AssertUnwindSafe(move || {
            kernel.lock().unwrap_or_else(|p| p.into_inner()).save_data()
        }));

        let result = match result {
            Ok(result) => result.map(From::from).map_err(From::from),
            Err(_) => {
                Err(save::SaveError::from(anyhow::anyhow!("inner kernel might be panicked")).into())
            }
        };

        if let Err(e) = result.as_ref() {
            log::error!("save data failed: {e}");
        }

        self.as_mut().rust_mut().result = Some(result);
        self.as_mut().data_saved();
    }

    pub fn get_load_result(&self) -> cxx_qt_lib::QMap<cxx_qt_lib::QMapPair_QString_QVariant> {
        let mut map = cxx_qt_lib::QMap::default();
        match self.result.as_ref() {
            Some(Ok(result::KernelResult::Load(result))) => {
                map.insert(
                    "level".into(),
                    cxx_qt_lib::QVariant::from(&(message_manager::MessageLevel::Positive as u8)),
                );
                let message = format!("Loading face '{id}' successfully!", id = result.id);
                log::info!("{}", crate::utils::lowercase_first_char(message.clone()));
                map.insert(
                    "message".into(),
                    cxx_qt_lib::QVariant::from(&cxx_qt_lib::QString::from(message)),
                );
            }
            Some(Err(e @ error::KernelError::Load(..))) => {
                map.insert(
                    "level".into(),
                    cxx_qt_lib::QVariant::from(&(message_manager::MessageLevel::Error as u8)),
                );
                let message = crate::utils::uppercase_first_char(format!("{e}"));
                log::error!("{}", crate::utils::lowercase_first_char(message.clone()));
                map.insert(
                    "message".into(),
                    cxx_qt_lib::QVariant::from(&cxx_qt_lib::QString::from(message)),
                );
            }
            _ => {
                log::error!("cannot get loading result");
                return map;
            }
        }

        map
    }

    pub fn get_match_result(&self) -> cxx_qt_lib::QMap<cxx_qt_lib::QMapPair_QString_QVariant> {
        let mut map = cxx_qt_lib::QMap::default();

        match self.result.as_ref() {
            Some(Ok(result::KernelResult::Match(result))) => {
                if result.score >= self.match_score {
                    map.insert(
                        "level".into(),
                        cxx_qt_lib::QVariant::from(
                            &(message_manager::MessageLevel::Positive as u8),
                        ),
                    );
                    let message = format!(
                        "Matching face '{id}' with score '{score:.2}' successfully",
                        id = result.best_id,
                        score = result.score
                    );
                    log::info!("{}", crate::utils::lowercase_first_char(message.clone()));
                    map.insert(
                        "message".into(),
                        cxx_qt_lib::QVariant::from(&cxx_qt_lib::QString::from(message)),
                    );
                } else {
                    map.insert(
                        "level".into(),
                        cxx_qt_lib::QVariant::from(&(message_manager::MessageLevel::Error as u8)),
                    );
                    let message = format!(
                        "Matching face '{id}' with score '{score:.2}' failed",
                        id = result.best_id,
                        score = result.score
                    );
                    log::error!("{}", crate::utils::lowercase_first_char(message.clone()));
                    map.insert(
                        "message".into(),
                        cxx_qt_lib::QVariant::from(&cxx_qt_lib::QString::from(message)),
                    );
                }
            }
            Some(Err(e @ error::KernelError::Match(..))) => {
                map.insert(
                    "level".into(),
                    cxx_qt_lib::QVariant::from(&(message_manager::MessageLevel::Error as u8)),
                );
                let message = crate::utils::uppercase_first_char(format!("{e}"));
                log::error!("{}", crate::utils::lowercase_first_char(message.clone()));
                map.insert(
                    "message".into(),
                    cxx_qt_lib::QVariant::from(&cxx_qt_lib::QString::from(message)),
                );
            }
            _ => {
                log::error!("cannot get matching result");
                return map;
            }
        }

        map
    }

    pub fn get_view_face_list_result(
        &self,
    ) -> cxx_qt_lib::QMap<cxx_qt_lib::QMapPair_QString_QVariant> {
        let mut map = cxx_qt_lib::QMap::default();

        match self.result.as_ref() {
            Some(Ok(result::KernelResult::ViewFaceList(result))) => {
                map.insert(
                    "level".into(),
                    cxx_qt_lib::QVariant::from(&(message_manager::MessageLevel::Positive as u8)),
                );
                let message = format!(
                    "Viewing list successfully, total {total} faces",
                    total = result.faces.len()
                );
                log::info!("{}", crate::utils::lowercase_first_char(message.clone()));
                map.insert(
                    "message".into(),
                    cxx_qt_lib::QVariant::from(&cxx_qt_lib::QString::from(message)),
                );
                let mut face_list = cxx_qt_lib::QList::default();
                for face in result.faces.iter() {
                    let mut item_map = cxx_qt_lib::QMap::default();
                    item_map.insert(
                        "createdAt".into(),
                        cxx_qt_lib::QVariant::from(&cxx_qt_lib::QString::from(
                            face.created_at.clone(),
                        )),
                    );
                    item_map.insert(
                        "faceId".into(),
                        cxx_qt_lib::QVariant::from(&cxx_qt_lib::QString::from(face.id.clone())),
                    );
                    face_list.append(cxx_qt_lib::QVariant::from(&item_map));
                }
                map.insert("faceList".into(), cxx_qt_lib::QVariant::from(&face_list));
            }
            Some(Err(e @ error::KernelError::ViewFaceList(..))) => {
                map.insert(
                    "level".into(),
                    cxx_qt_lib::QVariant::from(&(message_manager::MessageLevel::Error as u8)),
                );
                let message = crate::utils::uppercase_first_char(format!("{e}"));
                log::error!("{}", crate::utils::lowercase_first_char(message.clone()));
                map.insert(
                    "message".into(),
                    cxx_qt_lib::QVariant::from(&cxx_qt_lib::QString::from(message)),
                );
            }
            _ => {
                log::error!("cannot get viewing list result");
                return map;
            }
        }

        map
    }

    pub fn get_delete_result(&self) -> cxx_qt_lib::QMap<cxx_qt_lib::QMapPair_QString_QVariant> {
        let mut map = cxx_qt_lib::QMap::default();

        match self.result.as_ref() {
            Some(Ok(result::KernelResult::Delete(result))) => {
                map.insert(
                    "level".into(),
                    cxx_qt_lib::QVariant::from(&(message_manager::MessageLevel::Positive as u8)),
                );
                let message = format!("Deleting face '{id}' successfully!", id = result.id);
                log::info!("{}", crate::utils::lowercase_first_char(message.clone()));
                map.insert(
                    "message".into(),
                    cxx_qt_lib::QVariant::from(&cxx_qt_lib::QString::from(message)),
                );
            }
            Some(Err(e @ error::KernelError::Delete(..))) => {
                map.insert(
                    "level".into(),
                    cxx_qt_lib::QVariant::from(&(message_manager::MessageLevel::Error as u8)),
                );
                let message = crate::utils::uppercase_first_char(format!("{e}"));
                log::error!("{}", crate::utils::lowercase_first_char(message.clone()));
                map.insert(
                    "message".into(),
                    cxx_qt_lib::QVariant::from(&cxx_qt_lib::QString::from(message)),
                );
            }
            _ => {
                log::error!("cannot get deleting result");
                return map;
            }
        }
        map
    }

    pub fn get_save_result(&self) -> cxx_qt_lib::QMap<cxx_qt_lib::QMapPair_QString_QVariant> {
        let mut map = cxx_qt_lib::QMap::default();
        match self.result.as_ref() {
            Some(Ok(result::KernelResult::Save(result))) => {
                map.insert(
                    "level".into(),
                    cxx_qt_lib::QVariant::from(&(message_manager::MessageLevel::Positive as u8)),
                );
                let message = format!(
                    "Saving data to {path} successfully!",
                    path = result.path.display()
                );
                log::info!("{}", crate::utils::lowercase_first_char(message.clone()));
                map.insert(
                    "message".into(),
                    cxx_qt_lib::QVariant::from(&cxx_qt_lib::QString::from(message)),
                );
            }
            Some(Err(e @ error::KernelError::Save(..))) => {
                map.insert(
                    "level".into(),
                    cxx_qt_lib::QVariant::from(&(message_manager::MessageLevel::Error as u8)),
                );
                let message = crate::utils::uppercase_first_char(format!("{e}"));
                log::error!("{}", crate::utils::lowercase_first_char(message.clone()));
                map.insert(
                    "message".into(),
                    cxx_qt_lib::QVariant::from(&cxx_qt_lib::QString::from(message)),
                );
            }
            _ => {
                log::error!("cannot get saving result");
                return map;
            }
        }
        map
    }
}
