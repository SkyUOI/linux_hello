import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import org.kde.libkcm

Kirigami.FormLayout {
    required property CameraManager manager
    required property Capture capture
    property alias model: comboBox.model
    property alias index: comboBox.currentIndex

    QQC2.ComboBox {
        id: comboBox

        Kirigami.FormData.label: "cameras selection"
        Layout.fillWidth: true
        enabled: model.length > 1
        textRole: "description"

        onActivated: index => {
            const dev = model[index];
            capture.cameraDevice = dev;
            kcm.needsSave = true;
        }
    }

    QQC2.Switch {
        id: mirroredSwitcher

        Kirigami.FormData.label: "mirrored: "
        checked: manager.mirrored
        onToggled: {
            manager.mirrored = checked;
            kcm.needsSave = true;
        }
    }
}
