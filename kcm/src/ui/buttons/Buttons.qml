import QtQuick.Layouts
import QtQuick.Controls as QQC2
import org.kde.linuxhello
import org.kde.kirigami as Kirigami

RowLayout {

    required property CameraManager camera
    required property FaceRecognitionKernel kernel
    required property MessageManager message
    property LoadDialog loadDialog

    QQC2.Button {
        text: camera.running ? "stop preview" : "start preview"
        icon.name: "view-refresh"
        onClicked: {
            camera.running = !camera.running;
        }
    }

    QQC2.Button {
        text: "load face"
        icon.name: "face-smile"
        onClicked: loadDialog.open()
        enabled: camera.running && !kernel.busy
    }

    QQC2.Button {
        text: "match face"
        icon.name: "user-identity"
        enabled: camera.running && !kernel.busy
        onClicked: {
            kernel.matchFace();
            message.publishMessage(Kirigami.MessageType.Information, "Matching ...");
        }
    }

    QQC2.Button {
        text: "view list"
        icon.name: "view-list-details"
        enabled: !kernel.busy
        onClicked: {
            kernel.viewFaceList();
        }
    }
}
