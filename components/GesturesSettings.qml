import QtQuick
import QtQuick.Layouts
import qs.Commons
import qs.Ui

ColumnLayout {
    id: root
    property var controller: null
    spacing: 6
    Text {
        Layout.fillWidth: true
        text: "Trackpad gestures"
        textFormat: Text.PlainText
        font.family: Style.font.family
        font.pixelSize: 13
        color: Color.popups.text
    }
    Repeater {
        model: [
            {key: "all", label: "Workspace and desktop swipes"},
            {key: "workspace", label: "Workspace swipes only"},
            {key: "desktop", label: "Desktop swipes only"},
            {key: "reset", label: "Use configuration"}
        ]
        delegate: ActionButton {
            required property var modelData
            Layout.fillWidth: true
            text: modelData.label
            selected: !!root.controller && root.controller.mode === modelData.key
            enabled: !!root.controller && !root.controller.busy
            onClicked: root.controller.run(modelData.key)
        }
    }
    Text {
        Layout.fillWidth: true
        text: "Three fingers left/right switch workspaces. Four fingers down shows the desktop; four fingers up restores windows. Opt-in: detected conflicts restore your previous configuration. Use configuration removes Familiar’s gestures. Two-finger tap uses your system’s right-click setting; scrolling over a running app cycles its windows."
        textFormat: Text.PlainText
        wrapMode: Text.WordWrap
        font.family: Style.font.family
        font.pixelSize: 12
        color: Color.popups.text
    }
    Text {
        Layout.fillWidth: true
        text: !root.controller ? "Familiar service is unavailable." : root.controller.busy ? "Applying…" : root.controller.message
        visible: text.length > 0
        textFormat: Text.PlainText
        wrapMode: Text.WordWrap
        font.family: Style.font.family
        font.pixelSize: 12
        color: Color.popups.text
    }
    ActionButton {
        Layout.fillWidth: true
        text: "Refresh gesture preference"
        enabled: !!root.controller && !root.controller.busy
        onClicked: root.controller.run("status")
    }
}
