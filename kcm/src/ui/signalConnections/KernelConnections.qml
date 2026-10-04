import QtQuick
import org.kde.linuxhello
import org.kde.kirigami as Kirigami
import org.kde.linuxhello.listDialog

Connections {
    id: root

    required property FaceRecognitionKernel kernel
    required property MessageManager messageManager

    property ResultDialog resultDialog
    property ListDialog listDialog

    target: kernel

    function onFaceLoaded() {
        const result = kernel.getLoadResult();

        const level = result["level"];
        const message = result["message"];

        messageManager.publishMessage(level, message);

        resultDialog.setLevel(level);
        resultDialog.setText(message);
        resultDialog.setTitle("Loading Face Result");
        resultDialog.open();
    }

    function onFaceMatched() {
        const result = kernel.getMatchResult();

        const level = result["level"];
        const message = result["message"];

        messageManager.publishMessage(level, message);

        resultDialog.setLevel(level);
        resultDialog.setText(message);
        resultDialog.setTitle("Matching Face Result");
        resultDialog.open();
    }

    function onListGenerated() {
        const result = kernel.getViewFaceListResult();

        const level = result["level"];
        const message = result["message"];

        messageManager.publishMessage(level, message);

        if (level === Kirigami.MessageType.Positive) {
            const faceList = result["faceList"];
            listDialog.model = faceList;
            listDialog.currentIndex = -1;
            Qt.callLater(() => {
                listDialog.open();
            });
        } else {
            resultDialog.setLevel(level);
            resultDialog.setText(message);
            resultDialog.setTitle("Viewing List Result");
            resultDialog.open();
        }
    }

    function onFaceDeleted() {
        const result = kernel.getDeleteResult();

        const level = result["level"];
        const message = result["message"];

        messageManager.publishMessage(level, message);

        resultDialog.setLevel(level);
        resultDialog.setText(message);
        resultDialog.setTitle("Deleting Face Result");
        resultDialog.open();

        Qt.callLater(() => {
            kernel.viewFaceList();
        });
    }

    function onDataSaved() {
        const result = kernel.getSaveResult();

        const level = result["level"];
        const message = result["message"];

        messageManager.publishMessage(level, message);
    }
}
