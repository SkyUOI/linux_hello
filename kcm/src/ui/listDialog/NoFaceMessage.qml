import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2

Rectangle {
    id: root

    required property real rowHeight
    property ListContent listContent

    implicitHeight: rowHeight
    visible: listContent.count === 0

    QQC2.Label {
        anchors.fill: parent

        text: "No face ID found"
        horizontalAlignment: Text.AlignHCenter
        verticalAlignment: Text.AlignVCenter
    }
}
