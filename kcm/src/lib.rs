use qobject::{KPluginMetaData, QObject};

mod camera;
mod config;
mod env_vars;
mod kcm;
mod log_manager;

#[cxx_qt::bridge]
pub mod qobject {
    unsafe extern "C++" {
        include!("cxx-kde-frameworks/kquickconfigmodule.h");
        type KQuickConfigModule = cxx_kde_frameworks::kcmutils::KQuickConfigModule;

        include!("cxx-kde-frameworks/kpluginmetadata.h");
        type KPluginMetaData = cxx_kde_frameworks::kcoreaddons::KPluginMetaData;

        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    unsafe extern "RustQt" {
        #[qobject]
        #[base = KQuickConfigModule]
        type Kcm = super::KcmRust;

        #[qinvokable(cxx_override, cxx_virtual)]
        fn save(self: Pin<&mut Kcm>);

        #[qinvokable(cxx_override, cxx_virtual)]
        fn load(self: Pin<&mut Kcm>);

        #[qsignal]
        fn saved(self: Pin<&mut Kcm>);

        #[qsignal]
        fn loaded(self: Pin<&mut Kcm>);

    }

    unsafe extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(bool, running)]
        #[qproperty(bool, mirrored)]
        type CameraManager = super::CameraManagerRust;

        #[cxx_name = "currentDeviceId"]
        #[qinvokable]
        fn current_device_id(self: Pin<&mut CameraManager>) -> QString;

        #[cxx_name = "setDevice"]
        #[qinvokable]
        fn set_device(self: Pin<&mut CameraManager>, device_id: QString);

        #[qinvokable]
        fn init(self: Pin<&mut CameraManager>, kcm: *const Kcm);

        #[cxx_name = "saveConfig"]
        #[qinvokable]
        fn save_config(self: &CameraManager);
    }

    unsafe extern "RustQt" {

        #[qobject]
        #[qml_element]
        type LogManager = super::LogManagerRust;

        #[qinvokable]
        fn init(self: Pin<&mut LogManager>, kcm: *const Kcm);

        #[cxx_name = "reportErrorMessage"]
        #[qinvokable]
        fn report_error_message(self: &LogManager, message: QString);

        #[cxx_name = "reportInfoMessage"]
        #[qinvokable]
        fn report_info_message(self: &LogManager, message: QString);
    }

    impl
        cxx_qt::Constructor<
            (*mut QObject, KPluginMetaData),
            BaseArguments = (*mut QObject, KPluginMetaData),
        > for Kcm
    {
    }
}

pub type KcmRust = kcm::Kcm;

pub type CameraManagerRust = camera::CameraManager;

pub type LogManagerRust = log_manager::LogManager;
