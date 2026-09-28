use std::pin;

use crate::qobject;

#[derive(Default)]
pub struct FrameProcessor {}

impl qobject::FrameProcessor {
    /// # Safety
    ///
    /// use this function only after sink is constructed
    pub unsafe fn attach_frame_sink(self: pin::Pin<&mut Self>, sink: *mut cxx_qt::QObject) {
        if sink.is_null() {
            log::error!("sink is null");
            return;
        }
        unsafe { qobject::video_relay_attach(sink, 100) };
    }
}

pub fn relay_frame(width: i32, height: i32, stride: i32, data: &[u8]) {
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
