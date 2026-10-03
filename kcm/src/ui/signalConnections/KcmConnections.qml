import QtQuick
import org.kde.libkcm
import org.kde.libkcm.camera

Connections {
    id: root

    target: kcm

    required property CameraManager cameraManager
    required property FrameCapturer frame
    required property FaceRecognitionKernel kernel

    property SavingDialog savingDialog
    property Device device
    property Camera camera

    function onStartSaving() {
        savingDialog.open();
        Qt.callLater(() => {
            cameraManager.saveConfig();
            frame.saveConfig();
            kernel.saveData();
            savingDialog.close();
            kcm.saveConfig();
        });
    }

    function onLoaded() {
        Qt.callLater(() => {
            device.syncDevices();
            camera.index = device.getCurrentDeviceIndex();
            camera.cameraDevice = device.resolveDevice();
        });
    }
}
