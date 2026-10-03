import org.kde.kirigami as Kirigami

Kirigami.PromptDialog {
    id: root

    implicitWidth: Kirigami.Units.gridUnit * 15
    iconName: "dialog-information"

    title: "Waiting for Saving"
    subtitle: "Please wait while the data and configuration is saved."
}
