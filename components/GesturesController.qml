import QtQuick
import Quickshell.Io

// Shared by every settings surface; only explicit choices mutate configuration.
Item {
    id: root
    property string helper: Qt.resolvedUrl("../bin/familiar-desktop").toString().replace(/^file:\/\//, "")
    readonly property bool busy: adapter.running
    property string mode: ""
    property string message: ""
    function run(choice) {
        if (busy || ["status", "all", "workspace", "desktop", "reset"].indexOf(choice) < 0) return false
        message = ""
        adapter.command = [helper, "gestures", choice]
        adapter.running = true
        return true
    }
    Process {
        id: adapter
        objectName: "gesturesAdapter"
        stdout: StdioCollector { id: output; waitForEnd: true }
        onExited: function(code, status) {
            try {
                var result = JSON.parse(output.text)
                if (code !== 0 || result.state !== "ok" || ["all", "workspace", "desktop", "reset"].indexOf(result.mode) < 0) {
                    root.mode = ""
                    root.message = String(result.message || "Could not read or apply trackpad preference.").slice(0, 300)
                    return
                }
                root.mode = result.mode
                root.message = String(result.message || "").slice(0, 300)
            } catch (e) {
                root.mode = ""
                root.message = "Could not read trackpad preference. Check the installed Familiar backend."
            }
        }
    }
}
