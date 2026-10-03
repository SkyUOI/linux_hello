import QtQuick
import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami

ListView {
    id: root

    required property real rowHeight
    required property real createdAtWidth
    required property real faceIdWidth

    implicitHeight: Math.min(root.contentHeight, rowHeight * 8)
    clip: true
    spacing: 0

    onVisibleChanged: {
        if (visible) {
            currentIndex = -1;
        }
    }

    delegate: Rectangle {
        width: root.width
        height: rowHeight
        color: index === root.currentIndex ? Kirigami.Theme.highlightColor : (index % 2 === 0 ? "transparent" : Kirigami.Theme.alternateBackgroundColor)

        RowLayout {
            anchors.fill: parent
            anchors.leftMargin: Kirigami.Units.smallSpacing
            anchors.rightMargin: Kirigami.Units.smallSpacing
            spacing: 0

            QQC2.Label {
                Layout.preferredWidth: root.createdAtWidth

                text: modelData.createdAt
                horizontalAlignment: Text.AlignHCenter
                elide: Text.ElideRight
            }

            QQC2.Label {
                Layout.preferredWidth: root.faceIdWidth

                text: modelData.faceId
                horizontalAlignment: Text.AlignHCenter
                elide: Text.ElideRight
            }
        }

        MouseArea {
            anchors.fill: parent
            onClicked: {
                root.currentIndex = index;
            }
        }
    }
}
