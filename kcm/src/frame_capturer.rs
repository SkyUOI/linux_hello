use cxx_qt::CxxQtType;

use crate::{config, qobject};
use std::{cell, pin, rc, sync};

#[derive(Default)]
pub struct FrameCapturer {
    config: Option<rc::Rc<cell::RefCell<config::Config>>>,
    interval_ms: u32,
    pub last_frame: sync::Arc<sync::Mutex<Option<face_recognition_api::FaceIdImage>>>,
}

impl qobject::FrameCapturer {
    /// # Safety
    ///
    /// Only run this function after `sink` is constructed
    pub unsafe fn attach_frame_sink(self: pin::Pin<&mut Self>, sink: *mut cxx_qt::QObject) {
        if sink.is_null() {
            log::error!("sink is null");
            return;
        }
        let processor = unsafe { pin::Pin::into_inner_unchecked(self) } as *mut _;
        unsafe { qobject::video_relay_attach(processor, sink, 100) };
    }

    /// # Safety
    ///
    /// Only run this function after `kcm` is constructed.
    pub unsafe fn init(mut self: pin::Pin<&mut Self>, kcm: *mut qobject::Kcm) {
        if kcm.is_null() {
            log::error!("kcm is null");
            return;
        }
        let config = rc::Rc::clone(&unsafe { &*kcm }.config);
        self.as_mut().rust_mut().interval_ms = config.borrow().frame_processor_config.interval_ms;
        self.as_mut().rust_mut().config = Some(config);
        log::info!("frame processor has been constructed successfully");
    }

    pub fn save_config(&self) {
        log::info!("frame processor starts to save configuration");
        let Some(mut config) = self.config.as_ref().map(|config| config.borrow_mut()) else {
            log::error!("frame processor cannot get config");
            return;
        };
        config.frame_processor_config.interval_ms = self.interval_ms;
    }


}

/// # Safety
///
/// Only run this function after `frame_processor` is constructed.
pub unsafe fn relay_frame(
    frame_capturer: *mut qobject::FrameCapturer,
    width: i32,
    height: i32,
    stride: i32,
    data: &[u8],
) {
    if frame_capturer.is_null() {
        log::error!("frame processor is null");
        return;
    }
    log::trace!(
        "width: {width}, height: {height}, stride: {stride}, data len: {}",
        data.len()
    );
    let Ok(width) = u32::try_from(width) else {
        log::error!("width or height cannot be transferred into u32");
        return;
    };
    let Ok(height) = u32::try_from(height) else {
        log::error!("height cannot be transferred into u32");
        return;
    };
    let Ok(stride) = u32::try_from(stride) else {
        log::error!("stride cannot be transferred into u32");
        return;
    };
    let mut last_frame = match unsafe { &*frame_capturer }.last_frame.lock() {
        Ok(last_frame) => last_frame,
        Err(e) => {
            log::error!("last_frame has been poisoned: {e}");
            return;
        }
    };
    *last_frame = Some(face_recognition_api::FaceIdImage {
        width,
        height,
        stride,
        data: data.to_vec(),
    });
}
