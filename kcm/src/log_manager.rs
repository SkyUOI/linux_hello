use crate::{
    config::{self, log_manager_config},
    qobject,
};
use cxx_qt::CxxQtType as _;
use fern::colors;
use std::{cell, fs, path, pin, rc};

pub struct LogManager {
    log_level_filter: log_manager_config::LevelFilter,
    log_path: Option<path::PathBuf>,
    config: Option<rc::Rc<cell::RefCell<config::Config>>>,
}

impl Default for LogManager {
    fn default() -> Self {
        Self {
            log_level_filter: log_manager_config::LevelFilter::Debug,
            log_path: Default::default(),
            config: Default::default(),
        }
    }
}

impl qobject::LogManager {
    /// # Safety
    ///
    /// Only run this function after `kcm` is constructed.
    pub unsafe fn init(mut self: pin::Pin<&mut Self>, kcm: *const qobject::Kcm) {
        if kcm.is_null() {
            log::error!("kcm is null");
            return;
        }
        let config = rc::Rc::clone(&unsafe { &*kcm }.config);
        (
            self.as_mut().rust_mut().log_level_filter,
            self.as_mut().rust_mut().log_path,
        ) = (
            std::env::var(crate::env_vars::LOG_LEVEL)
                .ok()
                .and_then(|var| var.parse::<log_manager_config::LevelFilter>().ok())
                .unwrap_or(config.borrow().log_manager_config.log_level_filter),
            std::env::var(crate::env_vars::LOG_PATH)
                .ok()
                .map(|var| path::Path::new(&var).to_owned())
                .or(config.borrow().log_manager_config.log_file_path.clone()),
        );
        let colors: colors::ColoredLevelConfig = colors::ColoredLevelConfig::new()
            .info(colors::Color::Green)
            .debug(colors::Color::Blue);
        let mut dispatch = fern::Dispatch::new()
            .format(
                move |call_back: fern::FormatCallback<'_>, argument, record| {
                    let when = jiff::Zoned::now();
                    call_back.finish(format_args!(
                        "\x1B[{}m[{} {} {}:{}] {}\x1B[0m",
                        colors.get_color(&record.level()).to_fg_str(),
                        when.strftime("%Y-%m-%d %H:%M:%S%.3f %:z"),
                        record.level(),
                        record.file().unwrap_or("<unknown>"),
                        record.line().unwrap_or(0),
                        argument
                    ))
                },
            )
            .level(self.log_level_filter.into())
            .chain(std::io::stdout());
        if let Some(log_path) = self.log_path.as_ref()
            && let Ok(log_file) = fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(log_path)
        {
            println!("fern chains log file: {}", log_path.display());
            dispatch = dispatch.chain(log_file);
        }
        if let Err(_) = dispatch.apply() {
            log::warn!("log manager has been loaded, keep the existing logger");
        }
        self.as_mut().rust_mut().config = Some(config);
        log::info!("log manager has been load successfully");
    }

    pub fn report_error_log(&self, message: cxx_qt_lib::QString) {
        log::error!("{message}")
    }

    pub fn report_info_log(&self, message: cxx_qt_lib::QString) {
        log::info!("{message}")
    }
}
