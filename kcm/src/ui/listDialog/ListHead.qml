import QtQuick
import org.kde.kirigami as Kirigami
import QtQuick.Layouts
import QtQuick.Controls as QQC2

Rectangle {
    id: root

    required property real createdAtWidth
    required property real faceIdWidth
    required property real rowHeight

    implicitHeight: rowHeight
    color: Kirigami.Theme.alternateBackgroundColor

    Rectangle {
        anchors.top: parent.top
        implicitWidth: parent.width
        color: Kirigami.Theme.disabledTextColor
        height: 2
    }

    RowLayout {
        anchors.fill: parent
        spacing: 0

        QQC2.Label {
            Layout.preferredWidth: createdAtWidth

            text: "Created At"
            horizontalAlignment: Text.AlignHCenter
            font.bold: true
            elide: Text.ElideRight
        }

        QQC2.Label {
            Layout.preferredWidth: faceIdWidth

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
