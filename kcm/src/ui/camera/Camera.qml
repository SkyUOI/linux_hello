import QtQuick.Layouts
import org.kde.kirigami as Kirigami
import org.kde.libkcm

ColumnLayout {
    id: root

    required property CameraManager manager
    required property MessageManager message
    required property LogManager log

    property alias cameraAvailable: preview.cameraAvailable
    property alias model: controls.model
    property alias cameraDevice: capture.cameraDevice
    property alias index: controls.index
    property alias videoSink: preview.videoOutput.videoSink

    Capture {
        id: capture
        manager: root.manager
        message: root.message
        log: root.log

        videoOutput: preview.videoOutput
    }

    spacing: Kirigami.Units.gridUnit

    Preview {
        id: preview
        Layout.fillWidth: true
        manager: root.manager
        Layout.preferredHeight: Math.max(Math.round(width * 9.0 / 16.0), Kirigami.Units.gridUnit * 10)
    }

    Controls {
        id: controls
        manager: root.manager
        capture: capture
        Layout.fillWidth: true
    }
}
