import QtQuick
import Quickshell
import Quickshell.Wayland
import qs.Commons

PanelWindow {
    id: root
    required property var controller
    visible: controller.active && controller.opened
    color: "transparent"
    exclusionMode: ExclusionMode.Ignore
    anchors { top: true; bottom: true; left: true; right: true }
    WlrLayershell.namespace: "familiar-desktop-setup"
    WlrLayershell.layer: WlrLayer.Overlay
    WlrLayershell.keyboardFocus: visible ? WlrKeyboardFocus.Exclusive : WlrKeyboardFocus.None
    onVisibleChanged: if (visible) focusScope.forceActiveFocus()
    onBackingWindowVisibleChanged: if (backingWindowVisible && visible) focusScope.forceActiveFocus()
    FocusScope {
        id: focusScope
        anchors.fill: parent
        focus: root.visible
        Keys.onEscapePressed: event => { root.controller.opened = false; event.accepted = true }
        Rectangle {
            anchors.fill: parent
            color: "#99000000"
            MouseArea { anchors.fill: parent; onClicked: root.controller.opened = false }
        }
        Flickable {
            anchors.fill: parent
            contentHeight: Math.max(height, card.implicitHeight + Style.space(32))
            clip: true
            SetupCard {
                id: card
                controller: root.controller
                width: Math.max(0, Math.min(implicitWidth, parent.width - Style.space(32)))
                height: implicitHeight
                x: (parent.width - width) / 2
                y: Math.max(Style.space(16), (parent.height - height) / 2)
                onDismissed: root.controller.opened = false
            }
        }
    }
}
