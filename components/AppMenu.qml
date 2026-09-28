import QtQuick
import Quickshell
import Quickshell.Wayland
import qs.Commons
import qs.Ui
import "../DockModel.js" as DockModel

// A named, mouse-first window list. It uses the same output and layer as the
// dock, and only the card accepts pointer input outside the dock itself.
PanelWindow {
    id: menu
    required property var root
    required property var dockWindow
    readonly property var app: root.contextApp
    readonly property var windows: app && app.toplevels ? app.toplevels : []
    readonly property int rowHeight: 34
    readonly property int cardWidth: 260
    readonly property int visibleWindowCount: Math.min(8, windows.length)
    readonly property int cardHeight: 54 + visibleWindowCount * rowHeight + 1 + 4 * rowHeight + 10
    readonly property int dockOffset: root.slotSize + 2 * (Style.gapsOut || 5) + 8
    readonly property real appOffset: (root.hasLeftWidgets ? root.leftWidgetsWidth + root.leftSeparatorSize : 0) +
                                      (root.contextAppIndex + 0.5) * root.slotSize
    readonly property real screenWidth: dockWindow && dockWindow.screen ? dockWindow.screen.width : 1920
    readonly property real screenHeight: dockWindow && dockWindow.screen ? dockWindow.screen.height : 1080

    screen: dockWindow ? dockWindow.screen : null
    visible: !!app && root.dockRevealed && root.dockAvailable
    WlrLayershell.namespace: "omarchy-dock-app-menu"
    WlrLayershell.layer: root.overlayMode ? WlrLayer.Overlay : WlrLayer.Top
    WlrLayershell.keyboardFocus: visible ? WlrKeyboardFocus.OnDemand : WlrKeyboardFocus.None
    exclusionMode: ExclusionMode.Ignore
    color: "transparent"
    mask: Region { item: card }

    anchors {
        top: root.barPosition === "bottom" || root.isVertical
        bottom: root.barPosition === "top" || root.isVertical
        left: root.barPosition === "right" || !root.isVertical
        right: root.barPosition === "left" || !root.isVertical
    }
    margins {
        top: root.barPosition === "bottom" ? dockOffset : 0
        bottom: root.barPosition === "top" ? dockOffset : 0
        left: root.barPosition === "right" ? dockOffset : 0
        right: root.barPosition === "left" ? dockOffset : 0
    }
    implicitWidth: root.isVertical ? cardWidth : screenWidth
    implicitHeight: root.isVertical ? screenHeight : cardHeight

    onVisibleChanged: if (visible) card.forceActiveFocus()

    function dismiss() { root.contextAppId = ""; root.contextAppIndex = -1 }
    function chooseWindow(index) {
        root.restoreOrLaunchItem(app, index)
        dismiss()
    }
    function action(kind) {
        if (!app) return
        if (kind === "new") {
            DockModel.setPendingCliHint(app.appId || app.desktopId || "", root.knownWindows)
            DockModel.launchApp(root.shell, app, Util)
        } else if (kind === "pin") {
            root.setPinned(DockModel.togglePinned(root.pinnedIds, app.appId, root.maxDockItems))
        } else if (kind === "minimize") {
            root.minimizeItem(app, app.activeTopIndex || 0)
        } else if (kind === "close") {
            var index = app.activeTopIndex || 0
            if (windows[index] && typeof windows[index].close === "function") windows[index].close()
        }
        dismiss()
    }

    Rectangle {
        id: card
        width: menu.cardWidth
        height: menu.cardHeight
        x: menu.root.isVertical
            ? 0
            : Math.max(6, Math.min(menu.width - width - 6,
                (menu.width - menu.root.totalDockDimension) / 2 + menu.appOffset - width / 2))
        y: menu.root.isVertical
            ? Math.max(6, Math.min(menu.height - height - 6,
                (menu.height - menu.root.totalDockDimension) / 2 + menu.appOffset - height / 2))
            : 0
        radius: Math.min(12, menu.root.systemRounding)
        color: Color.popups.background
        border.width: Math.max(1, menu.root.systemBorderSize)
        border.color: Color.popups.border
        focus: true
        Keys.onEscapePressed: function(event) { menu.dismiss(); event.accepted = true }

        Column {
            anchors.fill: parent
            anchors.margins: 7
            spacing: 0

            Text {
                width: parent.width
                height: 40
                leftPadding: 9
                verticalAlignment: Text.AlignVCenter
                text: menu.app ? (menu.app.name || menu.app.appId) : ""
                elide: Text.ElideRight
                textFormat: Text.PlainText
                font.family: Style.font.family
                font.pixelSize: 14
                font.bold: true
                color: Color.popups.text
            }

            Flickable {
                width: parent.width
                height: menu.visibleWindowCount * menu.rowHeight
                contentWidth: width
                contentHeight: menu.windows.length * menu.rowHeight
                clip: true
                boundsBehavior: Flickable.StopAtBounds
                Column {
                    width: parent.width
                    Repeater {
                        model: menu.windows
                        delegate: Rectangle {
                            required property int index
                            required property var modelData
                            width: parent.width
                            height: menu.rowHeight
                            radius: 6
                            color: windowMouse.containsMouse ? Color.accent : "transparent"
                            Text {
                                anchors.fill: parent
                                anchors.leftMargin: 10
                                anchors.rightMargin: 10
                                verticalAlignment: Text.AlignVCenter
                                text: (menu.app && menu.app.isActive && menu.app.activeTopIndex === index ? "●  " : "    ") +
                                      (modelData.title || menu.app.name || "Untitled window")
                                elide: Text.ElideRight
                                textFormat: Text.PlainText
                                font.family: Style.font.family
                                font.pixelSize: 12
                                color: windowMouse.containsMouse ? Color.popups.background : Color.popups.text
                            }
                            MouseArea {
                                id: windowMouse
                                anchors.fill: parent
                                hoverEnabled: true
                                cursorShape: Qt.PointingHandCursor
                                onClicked: menu.chooseWindow(index)
                            }
                        }
                    }
                }
            }
            Rectangle { width: parent.width; height: 1; color: Color.popups.border }
            Repeater {
                model: [
                    { label: "New Window", kind: "new", enabled: true },
                    { label: menu.app && menu.app.isPinned ? "Unpin from Dock" : "Pin to Dock", kind: "pin", enabled: true },
                    { label: "Minimize Current Window", kind: "minimize", enabled: menu.windows.length > 0 },
                    { label: "Close Current Window", kind: "close", enabled: menu.windows.length > 0 }
                ]
                delegate: Rectangle {
                    required property var modelData
                    width: parent.width
                    height: menu.rowHeight
                    radius: 6
                    color: actionMouse.containsMouse && modelData.enabled ? Color.accent : "transparent"
                    Text {
                        anchors.fill: parent
                        anchors.leftMargin: 10
                        verticalAlignment: Text.AlignVCenter
                        text: modelData.label
                        textFormat: Text.PlainText
                        font.family: Style.font.family
                        font.pixelSize: 12
                        color: !modelData.enabled ? Color.popups.text :
                               actionMouse.containsMouse ? Color.popups.background : Color.popups.text
                        opacity: modelData.enabled ? 1 : 0.45
                    }
                    MouseArea {
                        id: actionMouse
                        anchors.fill: parent
                        enabled: modelData.enabled
                        hoverEnabled: true
                        cursorShape: Qt.PointingHandCursor
                        onClicked: menu.action(modelData.kind)
                    }
                }
            }
        }
    }
}
