use crate::{message_manager, qobject};
use cxx_qt::{CxxQtType, Threading};
use std::{panic, pin, sync};

pub type SharedKernel = sync::Arc<sync::Mutex<dyn face_recognition_api::FaceKernel + Send>>;

pub fn create_kernel() -> SharedKernel {
    #[cfg(feature = "noop-kernel")]
    {
        sync::Arc::new(sync::Mutex::new(face_recognition_api::NoopKernel))
    }
    #[cfg(feature = "face-id-kernel")]
    {
        todo!()
    }
}

#[derive(Default)]
pub struct FaceRecognitionKernel {
    kernel: Option<SharedKernel>,
    last_frame: sync::Arc<sync::Mutex<Option<face_recognition_api::FaceIdImage>>>,
    pub busy: bool,
    result: Option<Result<face_recognition_api::KernelResult, face_recognition_api::KernelError>>,
}

impl qobject::FaceRecognitionKernel {
    /// # Safety
    ///
    /// Only run this function after kcm is constructed.
    pub unsafe fn init(mut self: pin::Pin<&mut Self>, frame_capturer: *mut qobject::FrameCapturer) {
        if frame_capturer.is_null() {
            log::error!("frame capturer is null");
            return;
        }
        let kernel = create_kernel();
        self.as_mut().rust_mut().kernel = Some(kernel);
        self.as_mut().rust_mut().last_frame = unsafe { &*frame_capturer }.last_frame.clone();
        log::info!("face recognition kernel has been loaded");
    }

    fn get_kernel_with_last_frame(
        self: pin::Pin<&mut Self>,
    ) -> Option<(SharedKernel, face_recognition_api::FaceIdImage)> {
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
                    Err(_) => Err(face_recognition_api::LoadError::from(anyhow::anyhow!(
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
            } else {
                log::info!("loading face successfully");
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
                    Err(_) => Err(face_recognition_api::MatchError::from(anyhow::anyhow!(
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
                    Err(_) => Err(face_recognition_api::MatchError::from(anyhow::anyhow!(
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

    pub fn get_load_result(&self) -> cxx_qt_lib::QMap<cxx_qt_lib::QMapPair_QString_QVariant> {
        let mut map = cxx_qt_lib::QMap::default();
        match self.result.as_ref() {
            Some(Ok(face_recognition_api::KernelResult::Load(result))) => {
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
            Some(Err(e @ face_recognition_api::KernelError::Load(..))) => {
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
            Some(Ok(face_recognition_api::KernelResult::Match(result))) => {
                map.insert(
                    "level".into(),
                    cxx_qt_lib::QVariant::from(&(message_manager::MessageLevel::Positive as u8)),
                );
                let message = format!(
                    "Matching face '{id}' with correct rate '{score:.2}%' successfully",
                    id = result.best_id,
                    score = result.score * 100f32
                );
                log::info!("{}", crate::utils::lowercase_first_char(message.clone()));
                map.insert(
                    "message".into(),
                    cxx_qt_lib::QVariant::from(&cxx_qt_lib::QString::from(message)),
                );
            }
            Some(Err(e @ face_recognition_api::KernelError::Match(..))) => {
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
            Some(Ok(face_recognition_api::KernelResult::ViewFaceList(result))) => {
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
            Some(Err(e @ face_recognition_api::KernelError::ViewFaceList(..))) => {
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
}
