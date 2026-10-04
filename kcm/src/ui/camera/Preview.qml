import QtQuick
import QtMultimedia
import org.kde.kirigami as Kirigami
import org.kde.linuxhello

Rectangle {
    id: root

    required property CameraManager manager
    property bool cameraAvailable: false

    readonly property alias videoOutput: video

    implicitHeight: Math.max(Math.round(width * 9.0 / 16.0), Kirigami.Units.gridUnit * 10)

    color: Kirigami.Theme.backgroundColor

    VideoOutput {
        id: video

        anchors.fill: parent
        fillMode: VideoOutput.PreserveAspectCrop
        visible: manager.running
        mirrored: manager.mirrored
    }

    Kirigami.PlaceholderMessage {
        anchors.centerIn: parent
        width: parent.width - Kirigami.Units.largeSpacing * 4
        icon.name: cameraAvailable ? "camera-video" : "camera-off"
        text: cameraAvailable ? "preview stop" : "no available camera"
        explanation: cameraAvailable ? "click 'start preview'" : "check system camera"
        visible: !manager.running
    }
}
