use log::info;

use crate::{config, qobject};
use std::{cell, fs, io::Write, pin, rc};

pub struct Kcm {
    pub config: rc::Rc<cell::RefCell<config::Config>>,
}

impl Kcm {
    fn save_config(&self) -> Result<(), config::ConfigError> {
        let config_path =
            config::Config::get_config_path().ok_or(config::ConfigError::GetConfigPath)?;
        let config_content = toml::to_string(self.config.as_ref())?;
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut config_file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&config_path)?;
        let _ = config_file.write(config_content.as_bytes())?;
        Ok(())
    }
}

impl cxx_qt::Constructor<(*mut qobject::QObject, qobject::KPluginMetaData)> for qobject::Kcm {
    type NewArguments = ();

    type BaseArguments = (*mut qobject::QObject, qobject::KPluginMetaData);

    type InitializeArguments = ();

    fn route_arguments(
        arguments: Self::BaseArguments,
    ) -> (
        Self::NewArguments,
        Self::BaseArguments,
        Self::InitializeArguments,
    ) {
        ((), arguments, ())
    }

    fn new(_arguments: Self::NewArguments) -> <Self as cxx_qt::CxxQtType>::Rust {
        let config = rc::Rc::new(cell::RefCell::new(match config::Config::load() {
            Ok(config) => {
                println!("get configuration: {config:#?}");
                config
            }
            Err(e) => {
                eprintln!("configuration error and use default one: {}", e);
                config::Config::default()
            }
        }));
        Kcm { config }
    }
}

impl qobject::Kcm {
    pub fn save(mut self: pin::Pin<&mut Self>) {
        info!("kcm starts to save configuration and data");
        self.as_mut().start_saving();
        if let Err(e) = self.save_config() {
            log::error!("save configuration error: {e}");
            return;
        }
        info!("configuration and data saved");
    }

    pub fn load(self: pin::Pin<&mut Self>) {
        self.loaded();
    }
}
