use qobject::{KPluginMetaData, QObject};

mod camera;
mod config;
mod env_vars;
mod frame_capturer;
mod kcm;
mod kernel;
mod log_manager;
mod message_manager;
mod utils;

pub use frame_capturer::relay_frame;

#[cxx_qt::bridge]
pub mod qobject {
    unsafe extern "C++" {
        include!("cxx-kde-frameworks/kquickconfigmodule.h");
        type KQuickConfigModule = cxx_kde_frameworks::kcmutils::KQuickConfigModule;

        include!("cxx-kde-frameworks/kpluginmetadata.h");
        type KPluginMetaData = cxx_kde_frameworks::kcoreaddons::KPluginMetaData;

        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;

        include!("cxx-qt-lib/qmap.h");
        type QMap_QString_QVariant = cxx_qt_lib::QMap<cxx_qt_lib::QMapPair_QString_QVariant>;

        include!("kcm/src/video_relay.h");

        #[namespace = "kcm_video_relay"]
        #[cxx_name = "attach"]
        #[allow(clippy::missing_safety_doc)]
        unsafe fn video_relay_attach(
            frame_capturer: *mut FrameCapturer,
            sink: *mut QObject,
            interval_ms: u32,
        );
    }

    extern "Rust" {
        unsafe fn relay_frame(
            frame_capturer: *mut FrameCapturer,
            width: i32,
            height: i32,
            stride: i32,
            data: &[u8],
        );
    }

    unsafe extern "RustQt" {
        #[qobject]
        #[base = KQuickConfigModule]
        type Kcm = super::KcmRust;

        #[cxx_name = "saveConfig"]
        #[qinvokable]
        fn save_config(self: &Kcm);

        #[qinvokable(cxx_override, cxx_virtual)]
        fn save(self: Pin<&mut Kcm>);

        #[qinvokable(cxx_override, cxx_virtual)]
        fn load(self: Pin<&mut Kcm>);

        #[cxx_name = "startSaving"]
        #[qsignal]
        fn start_saving(self: Pin<&mut Kcm>);

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

        #[cxx_name = "reportErrorLog"]
        #[qinvokable]
        fn report_error_log(self: &LogManager, message: QString);

        #[cxx_name = "reportInfoLog"]
        #[qinvokable]
        fn report_info_log(self: &LogManager, message: QString);
    }

    unsafe extern "RustQt" {
        #[qobject]
        #[qml_element]
        type FrameCapturer = super::FrameCapturerRust;

        #[cxx_name = "attachFrameSink"]
        #[qinvokable]
        fn attach_frame_sink(self: Pin<&mut FrameCapturer>, sink: *mut QObject);

        #[qinvokable]
        fn init(self: Pin<&mut FrameCapturer>, kcm: *mut Kcm);

        #[cxx_name = "saveConfig"]
        #[qinvokable]
        fn save_config(self: &FrameCapturer);
    }

    unsafe extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(bool, busy)]
        type FaceRecognitionKernel = super::FaceRecognitionKernelRust;

        #[qinvokable]
        fn init(
            self: Pin<&mut FaceRecognitionKernel>,
            frame_capturer: *mut FrameCapturer,
            kcm: *mut Kcm,
        );

        #[cxx_name = "loadFace"]
        #[qinvokable]
        fn load_face(self: Pin<&mut FaceRecognitionKernel>, id: QString);

        #[cxx_name = "matchFace"]
        #[qinvokable]
        fn match_face(self: Pin<&mut FaceRecognitionKernel>);

        #[cxx_name = "viewFaceList"]
        #[qinvokable]
        fn view_face_list(self: Pin<&mut FaceRecognitionKernel>);

        #[cxx_name = "deleteFace"]
        #[qinvokable]
        fn delete_face(self: Pin<&mut FaceRecognitionKernel>, id: QString);

        #[cxx_name = "saveData"]
        #[qinvokable]
        fn save_data(self: Pin<&mut FaceRecognitionKernel>);

        #[cxx_name = "faceLoaded"]
        #[qsignal]
        fn face_loaded(self: Pin<&mut FaceRecognitionKernel>);

        #[cxx_name = "faceMatched"]
        #[qsignal]
        fn face_matched(self: Pin<&mut FaceRecognitionKernel>);

        #[cxx_name = "listGenerated"]
        #[qsignal]
        fn list_generated(self: Pin<&mut FaceRecognitionKernel>);

        #[cxx_name = "faceDeleted"]
        #[qsignal]
        fn face_deleted(self: Pin<&mut FaceRecognitionKernel>);

        #[cxx_name = "dataSaved"]
        #[qsignal]
        fn data_saved(self: Pin<&mut FaceRecognitionKernel>);

        #[cxx_name = "getLoadResult"]
        #[qinvokable]
        fn get_load_result(self: &FaceRecognitionKernel) -> QMap_QString_QVariant;

        #[cxx_name = "getMatchResult"]
        #[qinvokable]
        fn get_match_result(self: &FaceRecognitionKernel) -> QMap_QString_QVariant;

        #[cxx_name = "getViewFaceListResult"]
        #[qinvokable]
        fn get_view_face_list_result(self: &FaceRecognitionKernel) -> QMap_QString_QVariant;

        #[cxx_name = "getDeleteResult"]
        #[qinvokable]
        fn get_delete_result(self: &FaceRecognitionKernel) -> QMap_QString_QVariant;

        #[cxx_name = "getSaveResult"]
        #[qinvokable]
        fn get_save_result(self: &FaceRecognitionKernel) -> QMap_QString_QVariant;
    }

    unsafe extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(QString, current_text, cxx_name = "currentText")]
        #[qproperty(i8, current_level, cxx_name = "currentLevel", READ = get_current_level, NOTIFY = current_level_changed)]
        type MessageManager = super::MessageManagerRust;

        #[cxx_name = "publishMessage"]
        #[qinvokable]
        fn publish_message(self: Pin<&mut MessageManager>, level: u8, message: QString);

        #[cxx_name = "clearMessage"]
        #[qinvokable]
        fn clear_message(self: Pin<&mut MessageManager>);

        #[cxx_name = "getCurrentLevel"]
        #[qinvokable]
        fn get_current_level(self: &MessageManager) -> i8;

        #[qsignal]
        fn current_level_changed(self: Pin<&mut MessageManager>);

        #[cxx_name = "newMessagePublished"]
        #[qsignal]
        fn new_message_published(self: Pin<&mut MessageManager>);

        #[cxx_name = "messageCleared"]
        #[qsignal]
        fn message_cleared(self: Pin<&mut MessageManager>);
    }

    impl
        cxx_qt::Constructor<
            (*mut QObject, KPluginMetaData),
            BaseArguments = (*mut QObject, KPluginMetaData),
        > for Kcm
    {
    }

    impl cxx_qt::Threading for FaceRecognitionKernel {}
}

pub type KcmRust = kcm::Kcm;
pub type CameraManagerRust = camera::CameraManager;
pub type LogManagerRust = log_manager::LogManager;
pub type FrameCapturerRust = frame_capturer::FrameCapturer;
pub type FaceRecognitionKernelRust = kernel::FaceRecognitionKernel;
pub type MessageManagerRust = message_manager::MessageManager;
