import QtQuick
import QtTest
import "../components"

TestCase {
    name: "InputPreferenceController"
    Component { id: factory; InputPreferenceController { kind: "command" } }
    function test_explicitScopedChoiceAndFailure() {
        var item = createTemporaryObject(factory, this)
        var adapter = findChild(item, "inputPreferenceAdapter")
        compare(adapter.starts, 0)
        verify(!item.run("enable; bad"))
        verify(item.run("enable"))
        compare(adapter.command.slice(1), ["input-preference", "command", "enable"])
        verify(!item.run("reset"))
        adapter.complete('{"state":"failed","message":"Reload failed; restored"}', 1)
        compare(item.mode, "")
        verify(item.message.indexOf("Reload failed") >= 0)
        verify(item.run("reset"))
        adapter.complete('{"state":"ok","mode":"reset"}', 0)
        compare(item.mode, "reset")
        item.kind = "resize"
        verify(item.run("enable"))
        compare(adapter.command.slice(1), ["input-preference", "resize", "enable"])
        adapter.complete('{"state":"ok","mode":"enable"}', 0)
        compare(item.mode, "enable")
    }
}
