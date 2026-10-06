import QtQuick
import Quickshell.Io

Item {
    id: root
    required property string kind
    property string helper: Qt.resolvedUrl("../bin/familiar-desktop").toString().replace(/^file:\/\//, "")
    readonly property bool busy: adapter.running
    property string mode: ""
    property string message: ""
    function run(choice) {
        if (busy || ["resize", "command"].indexOf(kind) < 0 || ["enable", "reset", "status"].indexOf(choice) < 0) return false
        message = ""
        adapter.command = [helper, "input-preference", kind, choice]
        adapter.running = true
        return true
    }
    Process {
        id: adapter
        objectName: "inputPreferenceAdapter"
        stdout: StdioCollector { id: output; waitForEnd: true }
        onExited: function(code, status) {
            try {
                var result = JSON.parse(output.text)
                if (code !== 0 || result.state !== "ok" || ["enable", "reset"].indexOf(result.mode) < 0) {
                    root.mode = ""
                    root.message = String(result.message || "Could not apply preference.").slice(0, 300)
                    return
                }
                root.mode = result.mode
                root.message = String(result.message || "").slice(0, 300)
            } catch (e) {
                root.mode = ""
                root.message = "Could not read preference. Check the installed Familiar backend."
            }
        }
    }
}
