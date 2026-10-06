import QtQuick
import QtQuick.Layouts
import qs.Commons
import qs.Ui
import "../ShortcutLabels.js" as ShortcutLabels

ColumnLayout {
    id: root
    property var controller: null
    property string labelStyle: "standard"
    spacing: 6
    Text {
        Layout.fillWidth: true
        text: "Desktop mode"
        textFormat: Text.PlainText
        font.family: Style.font.family
        font.pixelSize: 13
        color: Color.popups.text
    }
    Repeater {
        model: [
            {key: "floating", label: "Floating · mouse-friendly overlapping windows"},
            {key: "tiling", label: "Tiling · let Hyprland arrange windows"},
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
        text: "Switches existing windows across regular workspaces and sets the layout for new windows, including " + ShortcutLabels.format("Super + Enter", root.labelStyle) + " terminals. Fullscreen, pinned, grouped and hidden windows are skipped. Tiling uses your existing Hyprland layout and keybindings. Use configuration removes the new-window override without moving existing windows."
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
        text: "Refresh window preference"
        enabled: !!root.controller && !root.controller.busy
        onClicked: root.controller.run("status")
    }
}
