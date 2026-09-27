import QtQuick
import QtQuick.Controls as QQC2
import QtQuick.Layouts
import org.kde.kcmutils as KCM
import org.kde.kirigami as Kirigami
import QtMultimedia
import org.kde.libkcm

KCM.SimpleKCM {
    id: root

    implicitWidth: Kirigami.Units.gridUnit * 20
    implicitHeight: Kirigami.Units.gridUnit * 40

    Component.onCompleted: {
        kcm.buttons = KCM.ConfigModule.NoAdditionalButton | KCM.ConfigModule.Apply | KCM.ConfigModule.Export;

        logManager.init(kcm);
        cameraManager.init(kcm);

        root.syncDevices();

        comboBox.currentIndex = root.getCurrentDeviceIndex();

        camera.cameraDevice = root.resolveDevice();
    }

    property bool initialized: false
    property bool saveAfterLoading: false
    property bool cameraAvailable: mediaDevices.videoInputs.length > 0

    readonly property CameraManager cameraManager: CameraManager {
        onRunningChanged: {
            camera.active = running;
        }
    }

    readonly property LogManager logManager: LogManager {}

    Connections {
        target: kcm

        function onSaved() {
            cameraManager.saveConfig();
        }

        function onLoaded() {
            if (root.saveAfterLoading) {
                kcm.needsSave = true;
                root.saveAfterLoading = false;
            }
        }
    }

    MediaDevices {
        id: mediaDevices

        onVideoInputsChanged: {
            Qt.callLater(root.syncDevices);
        }
    }

    function syncDevices() {
        const device = resolveDevice();
        if (String(device.id) === "") {
            logManager.reportErrorMessage("cannot resolve device");
            return;
        }
        if (cameraManager.currentDeviceId() !== String(device.id)) {
            if (root.initialized) {
                kcm.needsSave = true;
            }
            if (!root.initialized) {
                root.saveAfterLoading = true;
            }
            cameraManager.setDevice(device.id);
        }
        root.initialized = true;
    }

    function resolveDevice() {
        const list = mediaDevices.videoInputs;
        if (cameraManager.running) {
            const found = list.find(d => cameraManager.currentDeviceId() === String(d.id));
            if (found !== undefined) {
                return found;
            }
        }
        return mediaDevices.defaultVideoInput;
    }

    function getCurrentDeviceIndex() {
        const id = cameraManager.currentDeviceId();
        for (var i = 0; i < mediaDevices.videoInputs.length; ++i) {
            if (String(mediaDevices.videoInputs[i].id) === id) {
                logManager.reportInfoMessage("the index of current device in list's is " + i);
                return i;
            }
        }
        return;
    }

    CaptureSession {
        id: captureSession

        camera: Camera {
            id: camera
            onCameraDeviceChanged: {
                cameraManager.setDevice(cameraDevice.id);
            }
            onActiveChanged: {
                if (active) {
                    logManager.reportInfoMessage("camera is active");
                } else {
                    logManager.reportInfoMessage("camera is inactive");
                }
            }
            onErrorOccurred: (errorCode, errorMessage) => {
                logManager.reportErrorMessage(errorMessage);
            }
        }

        videoOutput: video
    }

    ColumnLayout {
        anchors.fill: parent
        spacing: Kirigami.Units.largeSpacing

        Rectangle {
            id: viewport

            Layout.fillWidth: true
            Layout.preferredHeight: Math.max(Math.round(width * 9.0 / 16.0), Kirigami.Units.gridUnit * 10)

            color: Kirigami.Theme.backgroundColor

            VideoOutput {
                id: video

                anchors.fill: parent
                fillMode: VideoOutput.PreserveAspectCrop
                visible: root.cameraManager.running
                mirrored: root.cameraManager.mirrored
            }

            Kirigami.PlaceholderMessage {
                anchors.centerIn: parent
                width: parent.width - Kirigami.Units.largeSpacing * 4
                icon.name: root.cameraAvailable ? "camera-video" : "camera-off"
                text: root.cameraAvailable ? "preview stop" : "no available camera"
                explanation: root.cameraAvailable ? "click 'start preview'" : "check system camera"
                visible: !root.cameraManager.running
            }
        }

        Kirigami.FormLayout {
            Layout.fillWidth: true

            QQC2.ComboBox {
                id: comboBox

                Kirigami.FormData.label: "cameras selection"
                Layout.fillWidth: true
                model: mediaDevices.videoInputs
                enabled: mediaDevices.videoInputs.length > 1
                textRole: "description"

                onActivated: index => {
                    const dev = mediaDevices.videoInputs[index];
                    root.cameraManager.setDevice(dev.id);
                    camera.cameraDevice = dev;
                    kcm.needsSave = true;
                }
            }

            QQC2.Switch {
                id: mirroredSwitcher

                Kirigami.FormData.label: "mirrored: "
                checked: root.cameraManager.mirrored
                onToggled: {
                    root.cameraManager.mirrored = checked;
                    kcm.needsSave = true;
                }
            }
        }

        RowLayout {
            Layout.fillWidth: true

            QQC2.Button {
                text: root.cameraManager.running ? "stop preview" : "start preview"
                icon.name: "view-refresh"
                onClicked: {
                    root.cameraManager.running = !root.cameraManager.running;
                }
            }
        }
    }
}
