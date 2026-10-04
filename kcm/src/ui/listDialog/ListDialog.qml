import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import org.kde.linuxhello

QQC2.Dialog {
    id: root

    property FaceRecognitionKernel kernel: FaceRecognitionKernel {}

    property DeleteDialog deleteDialog: DeleteDialog {
        id: deleteDialog
        anchors.centerIn: parent
        kernel: root.kernel
    }

    function init() {
        deleteDialog.listContent = listContent;
        noFaceMessage.listContent = listContent;
    }

    property alias model: listContent.model
    property alias currentIndex: listContent.currentIndex

    title: "Face ID List"
    modal: true

    footer: QQC2.DialogButtonBox {
        id: footer
        QQC2.Button {
            id: deleteButton

            text: "Delete"
            icon.name: "edit-delete"

            enabled: listContent.currentIndex !== -1
            onClicked: {
                root.deleteDialog.open();
            }
        }

        standardButtons: QQC2.Dialog.Ok
    }

    implicitWidth: Kirigami.Units.gridUnit * 20

    contentItem: ColumnLayout {
        id: listIdLayout

        anchors.left: parent.left
        anchors.right: parent.right

        readonly property real rowHeight: Kirigami.Units.gridUnit * 2
        readonly property real createdAtWidth: listContent.width * 0.4
        readonly property real faceIdWidth: listContent.width * 0.6

        spacing: 0

        ListHead {
            id: listHead

            Layout.fillWidth: true
            rowHeight: listIdLayout.rowHeight
            createdAtWidth: listIdLayout.createdAtWidth
            faceIdWidth: listIdLayout.faceIdWidth
        }

        ListContent {
            id: listContent
            Layout.fillWidth: true

            rowHeight: listIdLayout.rowHeight
            createdAtWidth: listIdLayout.createdAtWidth
            faceIdWidth: listIdLayout.faceIdWidth
        }

        Rectangle {
            implicitWidth: parent.width
            implicitHeight: 2
            color: Kirigami.Theme.disabledTextColor
        }

        NoFaceMessage {
            id: noFaceMessage
            Layout.fillWidth: true

            rowHeight: listIdLayout.rowHeight
        }
    }
}
