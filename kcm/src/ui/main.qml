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
        frameCapturer.init(kcm);

        faceRecognitionKernel.init(frameCapturer, kcm);

        frameCapturer.attachFrameSink(video.videoSink);

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
    readonly property FrameCapturer frameCapturer: FrameCapturer {}
    readonly property FaceRecognitionKernel faceRecognitionKernel: FaceRecognitionKernel {}
    readonly property MessageManager messageManager: MessageManager {}

    Connections {
        target: kcm

        function onStartSaving() {
            cameraManager.saveConfig();
            frameCapturer.saveConfig();
            faceRecognitionKernel.saveData();
        }

        function onLoaded() {
            if (root.saveAfterLoading) {
                kcm.needsSave = true;
                root.saveAfterLoading = false;
            }
        }
    }

    // Kirigami.PromptDialog {
    //     id: waitingSavingDialog

    //     anchors.centerIn: parent
    //     implicitWidth: Kirigami.Units.gridUnit * 15
    //     iconName: "dialog-information"

    //     title: "Waiting for Saving"
    //     subtitle: "Please wait while the data and configuration is saved."
    // }

    Connections {
        target: messageManager

        function onNewMessagePublished() {
            inlineMessage.visible = true;
        }

        function onMessageCleared() {
            inlineMessage.visible = false;
        }
    }

    Connections {
        target: faceRecognitionKernel

        function onFaceLoaded() {
            const result = faceRecognitionKernel.getLoadResult();

            const level = result["level"];
            const message = result["message"];

            messageManager.publishMessage(level, message);

            resultDialog.setLevel(level);
            resultDialog.setText(message);
            resultDialog.setTitle("Loading Face Result");
            resultDialog.open();
        }

        function onFaceMatched() {
            const result = faceRecognitionKernel.getMatchResult();

            const level = result["level"];
            const message = result["message"];

            messageManager.publishMessage(level, message);

            resultDialog.setLevel(level);
            resultDialog.setText(message);
            resultDialog.setTitle("Matching Face Result");
            resultDialog.open();
        }

        function onListGenerated() {
            const result = faceRecognitionKernel.getViewFaceListResult();

            const level = result["level"];
            const message = result["message"];

            messageManager.publishMessage(level, message);

            if (level === Kirigami.MessageType.Positive) {
                const faceList = result["faceList"];
                listView.model = faceList;
                listIdDialog.open();
            } else {
                resultDialog.setLevel(level);
                resultDialog.setText(message);
                resultDialog.setTitle("Viewing List Result");
                Qt.callLater(() => {
                    resultDialog.open();
                });
            }
        }

        function onFaceDeleted() {
            const result = faceRecognitionKernel.getDeleteResult();

            const level = result["level"];
            const message = result["message"];

            messageManager.publishMessage(level, message);

            resultDialog.setLevel(level);
            resultDialog.setText(message);
            resultDialog.setTitle("Deleting Face Result");
            resultDialog.open();

            listView.currentIndex = -1;
            faceRecognitionKernel.viewFaceList();
        }

        function onDataSaved() {
            const result = faceRecognitionKernel.getSaveResult();

            const level = result["level"];
            const message = result["message"];

            messageManager.publishMessage(level, message);
        }
    }

    Kirigami.PromptDialog {
        id: resultDialog

        anchors.centerIn: parent

        modal: true
        standardButtons: Kirigami.Dialog.Ok

        implicitWidth: Kirigami.Units.gridUnit * 20

        ColumnLayout {
            anchors.topMargin: Kirigami.Units.largeSpacing
            spacing: Kirigami.Units.largeSpacing

            Item {
                Layout.fillWidth: true
            }

            Kirigami.SelectableLabel {
                id: faceIdDialogText
                Layout.fillWidth: true

                font.pointSize: Kirigami.Theme.defaultFont.pointSize * 1.1
                wrapMode: Text.WordWrap
            }
        }

        function setTitle(text) {
            title = text;
        }

        function setText(text) {
            faceIdDialogText.text = text;
        }

        function setLevel(level) {
            switch (level) {
            case Kirigami.MessageType.Information:
                iconName = "dialog-information";
                break;
            case Kirigami.MessageType.Positive:
                iconName = "dialog-positive";
                break;
            case Kirigami.MessageType.Warning:
                iconName = "dialog-warning";
                break;
            case Kirigami.MessageType.Error:
                iconName = "dialog-error";
                break;
            default:
                iconName = "dialog-question";
                break;
            }
        }
    }

    QQC2.Dialog {
        id: listIdDialog

        anchors.centerIn: parent

        title: "Face ID List"
        modal: true
        standardButtons: QQC2.Dialog.Ok

        implicitWidth: Kirigami.Units.gridUnit * 20

        contentItem: ColumnLayout {
            id: listIdLayout

            // anchors.fill: parent
            anchors.left: parent.left
            anchors.right: parent.right

            readonly property real rowHeight: Kirigami.Units.gridUnit * 2
            readonly property real createdAtWidth: listView.width * 0.4
            readonly property real faceIdWidth: listView.width * 0.6

            spacing: 0

            Rectangle {
                Layout.fillWidth: true
                implicitHeight: listIdLayout.rowHeight
                color: Kirigami.Theme.alternateBackgroundColor

                Rectangle {
                    anchors.top: parent.top
                    implicitWidth: parent.width
                    color: Kirigami.Theme.disabledTextColor
                    height: 2
                }

                RowLayout {
                    anchors.fill: parent
                    // anchors.leftMargin: Kirigami.Units.smallSpacing
                    // anchors.rightMargin: Kirigami.Units.smallSpacing
                    spacing: 0

                    QQC2.Label {
                        Layout.preferredWidth: listIdLayout.createdAtWidth

                        text: "Created At"
                        horizontalAlignment: Text.AlignHCenter
                        font.bold: true
                        elide: Text.ElideRight
                    }

                    QQC2.Label {
                        Layout.preferredWidth: listIdLayout.faceIdWidth

                        text: "Face ID"
                        horizontalAlignment: Text.AlignHCenter
                        font.bold: true
                        elide: Text.ElideRight
                    }
                }

                Rectangle {
                    anchors.bottom: parent.bottom
                    implicitWidth: parent.width
                    height: 1
                    color: Kirigami.Theme.disabledTextColor
                }
            }

            ListView {
                id: listView
                Layout.fillWidth: true
                implicitHeight: Math.min(listView.contentHeight, listIdLayout.rowHeight * 8)
                clip: true
                spacing: 0
                onVisibleChanged: {
                    if (visible) {
                        currentIndex = -1;
                    }
                }

                delegate: Rectangle {
                    width: listView.width
                    height: listIdLayout.rowHeight
                    color: index === listView.currentIndex ? Kirigami.Theme.highlightColor : (index % 2 === 0 ? "transparent" : Kirigami.Theme.alternateBackgroundColor)

                    RowLayout {
                        anchors.fill: parent
                        anchors.leftMargin: Kirigami.Units.smallSpacing
                        anchors.rightMargin: Kirigami.Units.smallSpacing
                        spacing: 0

                        QQC2.Label {
                            Layout.preferredWidth: listIdLayout.createdAtWidth

                            text: modelData.createdAt
                            horizontalAlignment: Text.AlignHCenter
                            elide: Text.ElideRight
                        }

                        QQC2.Label {
                            Layout.preferredWidth: listIdLayout.faceIdWidth

                            text: modelData.faceId
                            horizontalAlignment: Text.AlignHCenter
                            elide: Text.ElideRight
                        }
                    }

                    MouseArea {
                        anchors.fill: parent
                        onClicked: {
                            listView.currentIndex = index;
                        }
                    }
                }

                Rectangle {
                    anchors.bottom: parent.bottom
                    implicitWidth: parent.width
                    height: 2
                    color: Kirigami.Theme.disabledTextColor
                }
            }

            Rectangle {
                Layout.fillWidth: true
                implicitHeight: listIdLayout.rowHeight
                visible: listView.count === 0

                QQC2.Label {
                    anchors.fill: parent

                    text: "No face ID found"
                    horizontalAlignment: Text.AlignHCenter
                    verticalAlignment: Text.AlignVCenter
                }
            }

            RowLayout {
                Layout.fillWidth: true
                Layout.topMargin: Kirigami.Units.gridUnit

                QQC2.Button {
                    text: "Delete"
                    icon.name: "edit-delete"

                    enabled: listView.currentIndex !== -1
                    onClicked: {
                        deletePromptDialog.open();
                    }
                }
            }
        }
    }

    Kirigami.PromptDialog {
        id: deletePromptDialog
        anchors.centerIn: parent
        implicitWidth: Kirigami.Units.gridUnit * 15

        title: "Delete Face ID"
        subtitle: "Delete face '%1'".arg(selectedFaceId())
        iconName: "edit-delete"

        standardButtons: Kirigami.Dialog.Ok | Kirigami.Dialog.Cancel

        onAccepted: {
            faceRecognitionKernel.deleteFace(selectedFaceId());
            kcm.needsSave = true;
        }

        function selectedFaceId() {
            return listView.currentIndex !== -1 ? String(listView.model[listView.currentIndex]?.faceId ?? "") : "";
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
            logManager.reportErrorLog("cannot resolve device");
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
                logManager.reportInfoLog("the index of current device in list's is " + i);
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
                    logManager.reportInfoLog("camera is active");
                    messageManager.clearMessage();
                } else {
                    logManager.reportInfoLog("camera is inactive");
                    messageManager.publishMessage(Kirigami.MessageType.Warning, "Camera is inactive, please start it");
                }
            }
            onErrorOccurred: (errorCode, errorMessage) => {
                logManager.reportErrorLog(errorMessage);
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
                visible: cameraManager.running
                mirrored: cameraManager.mirrored
            }

            Kirigami.PlaceholderMessage {
                anchors.centerIn: parent
                width: parent.width - Kirigami.Units.largeSpacing * 4
                icon.name: root.cameraAvailable ? "camera-video" : "camera-off"
                text: root.cameraAvailable ? "preview stop" : "no available camera"
                explanation: root.cameraAvailable ? "click 'start preview'" : "check system camera"
                visible: !cameraManager.running
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
                    cameraManager.setDevice(dev.id);
                    camera.cameraDevice = dev;
                    kcm.needsSave = true;
                }
            }

            QQC2.Switch {
                id: mirroredSwitcher

                Kirigami.FormData.label: "mirrored: "
                checked: cameraManager.mirrored
                onToggled: {
                    cameraManager.mirrored = checked;
                    kcm.needsSave = true;
                }
            }
        }

        RowLayout {
            Layout.fillWidth: true

            QQC2.Button {
                text: cameraManager.running ? "stop preview" : "start preview"
                icon.name: "view-refresh"
                onClicked: {
                    cameraManager.running = !cameraManager.running;
                }
            }

            QQC2.Button {
                text: "load face"
                icon.name: "face-smile"
                onClicked: faceIdDialog.open()
                enabled: cameraManager.running && !root.faceRecognitionKernel.busy
            }

            QQC2.Button {
                text: "match face"
                icon.name: "user-identity"
                enabled: cameraManager.running && !root.faceRecognitionKernel.busy
                onClicked: {
                    faceRecognitionKernel.matchFace();
                    messageManager.publishMessage(Kirigami.MessageType.Information, "Matching ...");
                }
            }

            QQC2.Button {
                text: "view list"
                icon.name: "view-list-details"
                enabled: !root.faceRecognitionKernel.busy
                onClicked: {
                    faceRecognitionKernel.viewFaceList();
                }
            }
        }

        Kirigami.InlineMessage {
            id: inlineMessage

            Layout.fillWidth: true
            text: messageManager.currentText
            type: levelToMessageType(messageManager.currentLevel)
            showCloseButton: true

            function levelToMessageType(level) {
                if (level == -1) {
                    return Kirigami.MessageType.Information;
                } else {
                    return level;
                }
            }

            Component.onCompleted: {
                visible = false;
            }
        }
    }

    QQC2.Dialog {
        id: faceIdDialog

        anchors.centerIn: parent

        title: "Input Face ID"
        modal: true
        standardButtons: QQC2.Dialog.Ok | QQC2.Dialog.Cancel

        implicitWidth: Kirigami.Units.gridUnit * 20

        QQC2.TextField {
            id: faceIdField
            anchors.fill: parent
            placeholderText: "please input face ID"
            onVisibleChanged: {
                if (visible) {
                    forceActiveFocus();
                    faceIdDialog.updateOkButton();
                }
            }
            onTextChanged: faceIdDialog.updateOkButton()
            onAccepted: {
                if (faceIdDialog.standardButton(QQC2.Dialog.Ok).enabled) {
                    faceIdDialog.accept();
                }
            }
        }

        function updateOkButton() {
            faceIdDialog.standardButton(QQC2.Dialog.Ok).enabled = faceIdField.length > 0;
        }

        onAccepted: {
            faceRecognitionKernel.loadFace(faceIdField.text);
            messageManager.publishMessage(Kirigami.MessageType.Information, "Loading ...");
            kcm.needsSave = true;
            faceIdField.text = "";
        }
    }
}
