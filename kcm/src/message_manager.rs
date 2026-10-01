use std::pin;

use cxx_qt::CxxQtType;

use crate::qobject;

#[derive(Default)]
pub struct MessageManager {
    current_level: Option<MessageLevel>,
    pub current_text: cxx_qt_lib::QString,
}

impl qobject::MessageManager {
    fn set_message_and_notify(mut self: pin::Pin<&mut Self>, message: cxx_qt_lib::QString) {
        self.as_mut().set_current_text(message);
        self.as_mut().current_level_changed();
        self.as_mut().new_message_published();
    }

    pub fn publish_message(mut self: pin::Pin<&mut Self>, level: u8, message: cxx_qt_lib::QString) {
        let level = match MessageLevel::try_from(level) {
            Ok(level) => level,
            Err(e) => {
                log::error!("publishing message type error: {e}");
                return;
            }
        };
        self.as_mut().rust_mut().current_level = Some(level);
        self.as_mut().set_message_and_notify(message);
    }

    pub fn clear_message(mut self: pin::Pin<&mut Self>) {
        self.as_mut().rust_mut().current_level = None;
        self.as_mut().set_current_text("".into());
        self.as_mut().current_level_changed();
        self.as_mut().message_cleared();
    }

    pub fn get_current_level(&self) -> i8 {
        self.current_level
            .as_ref()
            .map(|&level| level as u8 as i8)
            .unwrap_or(-1)
    }
}

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum MessageLevel {
    Information,
    Positive,
    Warning,
    Error,
}

impl From<MessageLevel> for u8 {
    fn from(value: MessageLevel) -> Self {
        value as Self
    }
}

impl TryFrom<u8> for MessageLevel {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, anyhow::Error> {
        match value {
            0 => Ok(Self::Information),
            1 => Ok(Self::Positive),
            2 => Ok(Self::Warning),
            3 => Ok(MessageLevel::Error),
            _ => anyhow::bail!("unknown message type"),
        }
    }
}
