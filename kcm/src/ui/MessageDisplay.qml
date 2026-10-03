import org.kde.kirigami as Kirigami
import QtQuick.Layouts
import org.kde.libkcm

Kirigami.InlineMessage {
    id: root

    required property MessageManager message

    text: message.currentText
    type: levelToMessageType(message.currentLevel)
    showCloseButton: true

    function levelToMessageType(level) {
        if (level == -1) {
            return Kirigami.MessageType.Information;
        } else {
            return level;
        }
    }


}
