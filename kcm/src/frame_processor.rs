use std::pin;

use crate::qobject;

#[derive(Default)]
pub struct FrameProcessor {}

impl qobject::FrameProcessor {
    pub unsafe fn attach_frame_sink(self: pin::Pin<&mut Self>, sink: *mut cxx_qt::QObject) {
        if sink.is_null() {
            log::error!("sink is null");
            return;
        }
        unsafe { qobject::video_relay_attach(sink, 100) };
    }
}

pub fn relay_frame(width: i32, height: i32, stride: i32, data: &[u8]) {}
