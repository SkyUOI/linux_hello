import QtQuick
import QtQuick.Layouts
import org.kde.kirigami as Kirigami

Kirigami.PromptDialog {
    id: root

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
