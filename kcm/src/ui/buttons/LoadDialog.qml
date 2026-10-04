import QtQuick.Controls as QQC2
import org.kde.kirigami as Kirigami
import org.kde.linuxhello

QQC2.Dialog {
    id: root

    required property MessageManager message
    required property FaceRecognitionKernel kernel

    title: "Input Face ID"
    modal: true
    standardButtons: QQC2.Dialog.Ok | QQC2.Dialog.Cancel

    implicitWidth: Kirigami.Units.gridUnit * 20

    QQC2.TextField {
        id: loadField
        anchors.fill: parent
        placeholderText: "please input face ID"
        onVisibleChanged: {
            if (visible) {
                forceActiveFocus();
                root.updateOkButton();
            }
        }
        onTextChanged: root.updateOkButton()
        onAccepted: {
            if (root.standardButton(QQC2.Dialog.Ok).enabled) {
                root.accept();
            }
        }
    }

    function updateOkButton() {
        root.standardButton(QQC2.Dialog.Ok).enabled = loadField.length > 0;
    }

    onAccepted: {
        kernel.loadFace(loadField.text);
        message.publishMessage(Kirigami.MessageType.Information, "Loading ...");
        kcm.needsSave = true;
        loadField.text = "";
    }
}
