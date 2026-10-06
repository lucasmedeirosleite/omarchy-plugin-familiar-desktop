import QtQuick
import QtTest
import "../components"

TestCase {
    name: "SetupCard"
    visible: true
    width: 800
    height: 800
    when: windowShown
    QtObject {
        id: model
        property bool ready: false
        property bool busy: false
        property bool canStart: true
        property string state: "needed"
        property string style: "windows"
        property string message: "One quick setup, then make yourself at home."
        property string details: "<img src=\"https://invalid.example/\">\nFailure details"
        property int starts: 0
        function start() { starts++ }
    }
    Component { id: card; SetupCard { controller: model } }
    function test_layoutFitsAtNormalAndCompactWidths() {
        var item = createTemporaryObject(card, this, {width: 540})
        verify(item !== null)
        wait(50)
        verify(item.implicitHeight > 200)
        verify(item.implicitHeight < 650)
        grabImage(item).save("/tmp/familiar-setup-preview.png")
        item.width = 350
        wait(50)
        verify(item.implicitHeight < 760)
    }
    function test_errorDetailsExpandAndSuccessShrinks() {
        var item = createTemporaryObject(card, this, {width: 540})
        model.state = "failed"
        item.showDetails = true
        wait(50)
        var failedHeight = item.implicitHeight
        model.state = "ready"
        model.ready = true
        wait(50)
        verify(item.implicitHeight < failedHeight)
        model.ready = false
        model.state = "needed"
    }
}
