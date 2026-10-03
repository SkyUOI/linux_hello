import QtQuick

import org.kde.libkcm

Connections {
    id: root

    required property MessageManager message
    
    property MessageDisplay messageDisplay

    target: message

    function onNewMessagePublished() {
        messageDisplay.visible = true;
    }

    function onMessageCleared() {
        messageDisplay.visible = false;
    }
}
