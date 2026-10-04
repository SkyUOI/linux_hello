import QtQuick
import org.kde.kirigami as Kirigami
import org.kde.linuxhello

Kirigami.PromptDialog {
    id: root
    implicitWidth: Kirigami.Units.gridUnit * 15

    required property FaceRecognitionKernel kernel
    property ListContent listContent

    title: "Delete Face ID"
    subtitle: "Delete face '%1'".arg(root.selectedFaceId)
    iconName: "edit-delete"

    standardButtons: Kirigami.Dialog.Ok | Kirigami.Dialog.Cancel

    property string selectedFaceId: listContent.currentIndex !== -1 ? String(listContent.model[listContent.currentIndex]?.faceId ?? "") : ""

    onAccepted: {
        kernel.deleteFace(selectedFaceId);
        kcm.needsSave = true;
    }
}
