// QtMultimedia is namespaced because the plain `Camera` type name would be
// shadowed by the Camera.qml file in the same directory, which QML's implicit
// directory import resolves with higher priority than explicit imports.
import QtMultimedia as MM
import org.kde.linuxhello
import org.kde.kirigami as Kirigami

MM.CaptureSession {
    id: root

    required property CameraManager manager
    required property MessageManager message
    required property LogManager log

    property alias cameraDevice: camera.cameraDevice

    camera: MM.Camera {
        id: camera
        onCameraDeviceChanged: {
            manager.setDevice(cameraDevice.id);
        }
        active: manager.running
        onActiveChanged: {
            if (active) {
                log.reportInfoLog("camera is active");
                message.clearMessage();
            } else {
                log.reportInfoLog("camera is inactive");
                message.publishMessage(Kirigami.MessageType.Warning, "Camera is inactive, please start it");
            }
        }
        onErrorOccurred: (errorCode, errorMessage) => {
            log.reportErrorLog(errorMessage);
        }
    }
}
