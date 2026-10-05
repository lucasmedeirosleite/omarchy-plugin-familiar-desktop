pragma Singleton
import QtQuick
QtObject {
 property var font: ({family: "DejaVu Sans"})
 property int cornerRadius: 12
 function space(v) { return v }
 function hoverFillFor(a,b) { return "#e5e7eb" }
}
