import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import qs.Commons

Rectangle {
    id: root
    required property var controller
    signal dismissed()
    implicitWidth: Style.space(540)
    implicitHeight: content.implicitHeight + Style.space(48)
    radius: Style.cornerRadius
    color: Color.popups.background
    border.color: Color.popups.border
    border.width: 1
    property bool showDetails: false
    MouseArea { anchors.fill: parent; acceptedButtons: Qt.AllButtons }

    ColumnLayout {
        id: content
        anchors { left: parent.left; right: parent.right; top: parent.top; margins: Style.space(24) }
        spacing: Style.space(16)
        RowLayout {
            Layout.fillWidth: true
            Text {
                text: "󰟀  Familiar"
                textFormat: Text.PlainText
                font.family: Style.font.family
                font.pixelSize: Style.space(16)
                font.bold: true
                color: Color.popups.text
                Layout.fillWidth: true
            }
            ActionButton {
                implicitWidth: 70
                text: "Close"
                Accessible.name: "Close setup; reopen from Familiar in the bar"
                onClicked: root.dismissed()
            }
        }
        Text {
            Layout.fillWidth: true
            text: root.controller.ready ? "Make yourself at home." : "Your desktop, a little more familiar."
            textFormat: Text.PlainText
            wrapMode: Text.WordWrap
            font.family: Style.font.family
            font.pixelSize: Style.space(25)
            font.bold: true
            color: Color.popups.text
        }
        Text {
            Layout.fillWidth: true
            text: root.controller.ready ? "The dock and window controls are ready. You can change your layout and preferences in Familiar at any time." : "Set up your dock and window controls. Familiar will download and verify the files it needs, then activate them here."
            textFormat: Text.PlainText
            wrapMode: Text.WordWrap
            font.family: Style.font.family
            font.pixelSize: Style.space(13)
            color: Color.popups.text
        }
        RowLayout {
            visible: !root.controller.ready
            enabled: !root.controller.busy && root.controller.canStart
            Layout.fillWidth: true
            Repeater {
                model: [{key: "windows", label: "Windows\nControls on the right"}, {key: "mac", label: "Mac\nControls on the left"}]
                delegate: ActionButton {
                    required property var modelData
                    Layout.fillWidth: true
                    implicitHeight: Style.space(64)
                    text: modelData.label
                    selected: root.controller.style === modelData.key
                    onClicked: root.controller.style = modelData.key
                }
            }
        }
        ProgressBar {
            Layout.fillWidth: true
            visible: root.controller.busy
            indeterminate: true
            Accessible.name: "Setup progress"
        }
        Text {
            Layout.fillWidth: true
            text: root.controller.message
            textFormat: Text.PlainText
            wrapMode: Text.WordWrap
            font.family: Style.font.family
            font.pixelSize: Style.space(12)
            color: Color.popups.text
        }
        ActionButton {
            Layout.fillWidth: true
            selected: true
            enabled: !root.controller.busy
            text: root.controller.ready ? "Start using Familiar" : root.controller.state === "failed" ? "Retry setup" : "Set up Familiar"
            onClicked: {
                if (root.controller.ready) root.dismissed()
                else root.controller.start()
            }
        }
        ActionButton {
            visible: root.controller.state === "failed"
            Layout.fillWidth: true
            text: root.showDetails ? "Hide details" : "Show details"
            onClicked: root.showDetails = !root.showDetails
        }
        ScrollView {
            visible: root.showDetails && root.controller.state === "failed"
            Layout.fillWidth: true
            Layout.preferredHeight: Style.space(100)
            clip: true
            TextArea {
                text: root.controller.details
                textFormat: TextEdit.PlainText
                wrapMode: TextEdit.Wrap
                readOnly: true
                selectByMouse: true
                color: Color.popups.text
                font.pixelSize: Style.space(11)
            }
        }
    }
}
