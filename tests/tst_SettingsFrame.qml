import QtQuick
import QtTest
import "../components"

Item {
    width: 800
    height: 560
TestCase {
    name: "SettingsFrame"
    visible: true
    width: 800
    height: 560
    when: windowShown
    SettingsFrame {
        id: frame
        width: 800
        height: 560
        contentHeight: 900
        Rectangle { width: parent.width; height: 900; color: "transparent" }
    }
    SignalSpy { id: dismissal; target: frame; signalName: "dismissed" }
    function test_navigation_and_scroll_reset() {
        var scroll = findChild(frame, "settings-scroll")
        scroll.contentY = 200
        var keyboard = findChild(frame, "settings-nav-keyboard")
        wait(30)
        mouseClick(keyboard)
        compare(frame.page, "keyboard")
        compare(scroll.contentY, 0)
        keyboard.forceActiveFocus()
        keyClick(Qt.Key_Up)
        compare(frame.page, "windows")
        verify(findChild(frame, "settings-nav-windows").activeFocus)
    }
    function test_compact_and_close() {
        frame.width = 560
        tryCompare(frame, "compact", true)
        wait(30)
        mouseClick(findChild(frame, "settings-nav-dock"))
        compare(frame.page, "dock")
        mouseClick(findChild(frame, "settings-close"))
        compare(dismissal.count, 1)
        frame.width = 800
    }
}

}
