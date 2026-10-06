import QtQuick
import QtTest
import "../components"

TestCase {
    name: "GesturesController"
    Component { id: factory; GesturesController {} }
    function test_explicitChoiceAndSerializedCommands() {
        var item = createTemporaryObject(factory, this)
        var adapter = findChild(item, "gesturesAdapter")
        compare(adapter.starts, 0)
        verify(!item.run("normal; touch /tmp/no"))
        compare(adapter.starts, 0)
        verify(item.run("status"))
        compare(adapter.command.slice(1), ["gestures", "status"])
        adapter.complete('{"state":"ok","mode":"reset"}', 0)
        compare(item.mode, "reset")
        verify(item.run("all"))
        verify(!item.run("all"))
        compare(item.mode, "reset") // no optimistic success before the backend responds
        compare(adapter.command.slice(1), ["gestures", "all"])
        adapter.complete('{"state":"ok","mode":"all","message":"Applied"}', 0)
        compare(item.mode, "all")
    }
    function test_failureMalformedOutputAndRetry() {
        var item = createTemporaryObject(factory, this)
        var adapter = findChild(item, "gesturesAdapter")
        item.run("all")
        adapter.complete('{"state":"failed","message":"Reload failed; restored"}', 1)
        compare(item.mode, "")
        verify(item.message.indexOf("Reload failed") >= 0)
        verify(item.run("status"))
        adapter.complete('not json', 0)
        compare(item.mode, "")
        verify(item.run("reset"))
        adapter.complete('{"state":"ok","mode":"reset"}', 0)
        compare(item.mode, "reset")
    }
    function test_unknownModeIsNotSuccess() {
        var item = createTemporaryObject(factory, this)
        var adapter = findChild(item, "gesturesAdapter")
        item.run("status")
        adapter.complete('{"state":"ok","mode":"unknown"}', 0)
        compare(item.mode, "")
        verify(item.message.length > 0)
    }
}
