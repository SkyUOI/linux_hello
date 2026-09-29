use cxx_qt::CxxQtType;

use crate::{config, qobject};
use std::{cell, pin, rc};

#[derive(Default)]
pub struct FrameProcessor {
    config: Option<rc::Rc<cell::RefCell<config::Config>>>,
    interval_ms: u32,
}

impl qobject::FrameProcessor {
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
    frame_processor: *mut qobject::FrameProcessor,
    width: i32,
    height: i32,
    stride: i32,
    data: &[u8],
) {
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
    let mut gray_image = image::GrayImage::new(width, height);

    for (i, row) in (0..height as usize).zip(gray_image.rows_mut()) {
        let row_start = i * stride as usize;
        row.zip(&data[row_start..][..width as usize])
            .for_each(|(pixel, &data_pixel)| *pixel = [data_pixel].into());
    }

    gray_image.save("./kcm/test_image/test.png").unwrap();
}
