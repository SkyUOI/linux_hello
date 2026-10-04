import QtQuick
import org.kde.linuxhello
import org.kde.kirigami as Kirigami

QtObject {
    id: root

    required property CameraManager cameraManager
    required property FrameCapturer frame
    required property FaceRecognitionKernel kernel
    required property MessageManager message

    property list<QtObject> connections: [
        KcmConnections {
            id: kcmConnections

            cameraManager: root.cameraManager
            kernel: root.kernel
            frame: root.frame
        },
        MessageConnections {
            id: messageConnections

            message: root.message
        },
        KernelConnections {
            id: kernelConnections

            kernel: root.kernel
            messageManager: root.message
        }
    ]

    function initDevice(device) {
        kcmConnections.device = device;
    }

    function initSavingDialog(savingDialog) {
        kcmConnections.savingDialog = savingDialog;
    }

    function initMessageDisplay(messageDisplay) {
        messageConnections.messageDisplay = messageDisplay;
    }

    function initResultDialog(resultDialog) {
        kernelConnections.resultDialog = resultDialog;
    }

    function initListDialog(listDialog) {
        kernelConnections.listDialog = listDialog;
    }

    function initCamera(camera) {
        kcmConnections.camera = camera;
    }
}
