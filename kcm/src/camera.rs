use crate::{config, qobject};
use cxx_qt::CxxQtType as _;
use log::info;
use std::{cell, io, path, pin, rc};

pub struct Camera {
    pub system_path: path::PathBuf,
    lists: udev::Enumerator,
}

impl Camera {
    pub fn new(system_path: path::PathBuf) -> Result<Self, CameraError> {
        let lists = udev::Enumerator::new();
        match lists {
            Ok(mut lists) => {
                lists.match_subsystem(CAMERA_SUBSYSTEM)?;
                let devices = lists.scan_devices()?;
                for device in devices {
                    if device.syspath().to_path_buf() == system_path {
                        return Ok(Self { system_path, lists });
                    }
                }
                Err(CameraError::FindSystemPath)
            }
            Err(e) => Err(e.into()),
        }
    }

    fn get_id(&mut self) -> Result<String, CameraError> {
        self.lists.match_subsystem(CAMERA_SUBSYSTEM)?;
        let devices = self.lists.scan_devices()?;

        for device in devices {
            if device.syspath() == self.system_path.as_os_str() {
                return Ok(device
                    .devnode()
                    .ok_or(CameraError::GetInformation)?
                    .to_str()
                    .ok_or(CameraError::ConvertId)?
                    .to_owned());
            }
        }

        Err(CameraError::GetId(self.system_path.clone()))
    }

    fn from_id(id: String) -> Result<Self, CameraError> {
        let mut lists = udev::Enumerator::new()?;
        lists.match_subsystem(CAMERA_SUBSYSTEM)?;

        let devices = lists.scan_devices()?;
        for device in devices {
            if device
                .devnode()
                .ok_or(CameraError::GetInformation)?
                .to_str()
                .ok_or(CameraError::ConvertId)?
                == id
            {
                return Ok(Self {
                    system_path: device.syspath().to_owned(),
                    lists,
                });
            }
        }

        Err(CameraError::FindId(id))
    }

    fn set_id(&mut self, id: String) -> Result<(), CameraError> {
        self.lists.match_subsystem(CAMERA_SUBSYSTEM)?;

        let devices = self.lists.scan_devices()?;
        for device in devices {
            if device
                .devnode()
                .ok_or(CameraError::GetInformation)?
                .to_str()
                .ok_or(CameraError::ConvertId)?
                == id
            {
                self.system_path = device.syspath().to_path_buf();
                return Ok(());
            }
        }
        Err(CameraError::FindId(id))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum CameraError {
    #[error("filesystem io error")]
    Io(#[from] io::Error),
    #[error("cannot get device information")]
    GetInformation,
    #[error("device's id cannot convert to string")]
    ConvertId,
    #[error("cannot find device's id '{0}'")]
    FindId(String),
    #[error("cannot find device's system path")]
    FindSystemPath,
    #[error("cannot get id through system path '{}'", .0.display())]
    GetId(path::PathBuf),
}

const CAMERA_SUBSYSTEM: &str = "video4linux";

#[derive(Default)]
pub struct CameraManager {
    camera: Option<Camera>,
    config: Option<rc::Rc<cell::RefCell<config::Config>>>,
    pub running: bool,
    pub mirrored: bool,
}

impl qobject::CameraManager {
    pub fn current_device_id(mut self: pin::Pin<&mut Self>) -> cxx_qt_lib::QString {
        self.as_mut()
            .rust_mut()
            .camera
            .as_mut()
            .map(|camera| match camera.get_id() {
                Ok(id) => id,
                Err(CameraError::GetId(_)) => "".to_owned(),
                Err(e) => panic!("{1}: {:?}", e.to_string(), "camera gets id sucessfully"),
            })
            .unwrap_or("".to_owned())
            .into()
    }

    pub fn set_device(mut self: pin::Pin<&mut Self>, device_id: cxx_qt_lib::QString) {
        let device_id = device_id.into();
        let camera = &mut self.as_mut().rust_mut().camera;
        if camera.as_mut().is_some_and(|camera| {
            camera.get_id().expect("camera gets id successfully") == device_id
        }) {
            return;
        }
        log::info!("set device successfully with id '{}'", device_id);
        match camera {
            Some(camera) => {
                if let Err(e) = camera.set_id(device_id) {
                    log::error!("set device id error: {e}");
                    self.set_running(false);
                }
            }
            None => {
                *camera = Some(match Camera::from_id(device_id) {
                    Ok(camera) => camera,
                    Err(e) => {
                        log::error!("device's id is available: {e}");
                        return;
                    }
                });
                self.set_running(true);
            }
        }
    }

    /// # Safety
    ///
    /// Only run this function after `kcm` is constructed.
    pub unsafe fn init(mut self: pin::Pin<&mut Self>, kcm: *const qobject::Kcm) {
        if kcm.is_null() {
            log::error!("kcm is null");
            self.set_running(false);
            return;
        }

        let config = rc::Rc::clone(&unsafe { &*kcm }.config);

        if let Some(system_path) = config.borrow().camera_config.camera_system_path.as_ref() {
            self.as_mut().rust_mut().camera = Some(match Camera::new(system_path.clone()) {
                Ok(camera) => {
                    self.as_mut().set_running(true);
                    camera
                }
                Err(e) => {
                    log::error!("camera construction failed: {e}");
                    return;
                }
            });
        }
        self.as_mut()
            .set_mirrored(config.borrow().camera_config.mirrored);
        self.as_mut()
            .set_running(config.borrow().camera_config.running);
        self.as_mut().rust_mut().config = Some(config);
        log::info!("camera manager has been load successfully");
    }

    pub fn save_config(&self) {
        info!("camera manager starts to save configuration");
        let Some(mut config) = self
            .config
            .as_ref()
            .map(|config| config.borrow_mut())
        else {
            log::error!("camera manager cannot get configuration");
            return;
        };
        config.camera_config.camera_system_path = self
            .camera
            .as_ref()
            .map(|camera| camera.system_path.clone());
        config.camera_config.mirrored = self.mirrored;
        config.camera_config.running = self.running;
    }
}
