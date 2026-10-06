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
        text: "New window layout"
        textFormat: Text.PlainText
        font.family: Style.font.family
        font.pixelSize: 13
        color: Color.popups.text
    }
    Repeater {
        model: [
            {key: "floating", label: "Floating · keep existing tiles the same size"},
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
        text: "Opt-in for all newly opened app windows, including Super+Enter terminals. Windows overlap instead of splitting tiles. Existing windows are unchanged. Use configuration removes this preference; use the dock’s Return to tiling action for existing windows."
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
