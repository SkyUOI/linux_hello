import QtMultimedia
import org.kde.linuxhello
import org.kde.kirigami as Kirigami

CaptureSession {
    id: root

    required property CameraManager manager
    required property MessageManager message
    required property LogManager log

    property alias cameraDevice: camera.cameraDevice

    camera: Camera {
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
