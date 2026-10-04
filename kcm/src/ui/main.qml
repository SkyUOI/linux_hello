import QtQuick
import QtQuick.Controls as QQC2
import QtQuick.Layouts
import org.kde.kcmutils as KCM
import org.kde.kirigami as Kirigami
import QtMultimedia
import org.kde.linuxhello
import org.kde.linuxhello.listDialog
import org.kde.linuxhello.camera
import org.kde.linuxhello.buttons
import org.kde.linuxhello.signalConnections

KCM.SimpleKCM {
    id: root

    implicitWidth: Kirigami.Units.gridUnit * 20
    implicitHeight: Kirigami.Units.gridUnit * 40

    Component.onCompleted: {
        kcm.buttons = KCM.ConfigModule.NoAdditionalButton | KCM.ConfigModule.Apply | KCM.ConfigModule.Export;

        buttons.loadDialog = loadDialog;

        signalConnections.initDevice(device);
        signalConnections.initResultDialog(resultDialog);
        signalConnections.initListDialog(listDialog);
        signalConnections.initMessageDisplay(messageDisplay);
        signalConnections.initSavingDialog(savingDialog);
        signalConnections.initCamera(camera);
        listDialog.init();

        logManager.init(kcm);
        cameraManager.init(kcm);
        frameCapturer.init(kcm);
        faceRecognitionKernel.init(frameCapturer, kcm);

        frameCapturer.attachFrameSink(camera.videoSink);
    }

    property bool initialized: false
    property bool saveAfterLoading: false
    property bool cameraAvailable: device.videoInputs.length > 0

    readonly property CameraManager cameraManager: CameraManager {}
    readonly property LogManager logManager: LogManager {}
    readonly property FrameCapturer frameCapturer: FrameCapturer {}
    readonly property FaceRecognitionKernel faceRecognitionKernel: FaceRecognitionKernel {}
    readonly property MessageManager messageManager: MessageManager {}

    SignalConnections {
        id: signalConnections

        cameraManager: root.cameraManager
        kernel: faceRecognitionKernel
        message: messageManager
        frame: frameCapturer
    }

    SavingDialog {
        id: savingDialog
        anchors.centerIn: parent
    }

    ResultDialog {
        id: resultDialog
        anchors.centerIn: parent
    }

    ListDialog {
        id: listDialog
        anchors.centerIn: parent
        kernel: faceRecognitionKernel
    }

    Device {
        id: device

        camera: cameraManager
        log: logManager

        onVideoInputsChanged: {
            if (root.initialized) {
                Qt.callLater(syncDevices);
            }
            root.initialized = true;
        }
    }

    ColumnLayout {
        anchors.fill: parent
        spacing: Kirigami.Units.largeSpacing

        Camera {
            id: camera
            Layout.fillWidth: true
            manager: cameraManager
            message: messageManager
            log: logManager
            cameraAvailable: root.cameraAvailable
            model: device.videoInputs
        }

        Buttons {
            id: buttons
            Layout.fillWidth: true
            camera: cameraManager
            kernel: faceRecognitionKernel
            message: messageManager
        }

        MessageDisplay {
            id: messageDisplay
            Layout.fillWidth: true

            message: messageManager
            Component.onCompleted: {
                visible = false;
            }
        }
    }

    LoadDialog {
        id: loadDialog
        message: messageManager
        kernel: faceRecognitionKernel
        anchors.centerIn: parent
    }
}
