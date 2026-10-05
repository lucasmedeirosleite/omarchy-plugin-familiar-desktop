pragma Singleton
import QtQuick
QtObject {
 property var popups: ({background: "#f6f7f9", text: "#202833", border: "#cbd0d7"})
 property color accent: "#006abe"
 property color background: "#ffffff"
 property color text: "#202833"
 property color muted: "#687281"
 property color border: "#cbd0d7"
 function composed(a,b,c,d) { return "#e5e7eb" }
}
