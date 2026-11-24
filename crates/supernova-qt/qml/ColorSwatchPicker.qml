import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

RowLayout {
    id: picker
    property string selectedColor: "#FFFF00"
    property bool controlsEnabled: true
    signal colorChosen(string hexColor)

    readonly property var swatches: [
        "#FFFF00",
        "#0000FF",
        "#FF00FF",
        "#00FF00",
        "#FF0000",
        "#00FFFF",
        "#E1BD22",
        "#FFFFFF"
    ]

    spacing: 8

    Repeater {
        model: picker.swatches
        delegate: Rectangle {
            required property string modelData
            width: 24
            height: 24
            radius: 12
            color: modelData
            border.width: picker.selectedColor.toUpperCase() === modelData ? 3 : 1
            border.color: picker.selectedColor.toUpperCase() === modelData ? "#FFFFFF" : "#444454"
            opacity: picker.controlsEnabled ? 1.0 : 0.45

            MouseArea {
                anchors.fill: parent
                enabled: picker.controlsEnabled
                cursorShape: enabled ? Qt.PointingHandCursor : Qt.ArrowCursor
                onClicked: picker.colorChosen(modelData)
            }
        }
    }

    TextField {
        id: hexInput
        Layout.preferredWidth: 96
        Layout.preferredHeight: 30
        enabled: picker.controlsEnabled
        text: picker.selectedColor
        color: "#F4F4F6"
        font.family: "monospace"
        font.pixelSize: 12
        horizontalAlignment: Text.AlignHCenter
        maximumLength: 7
        selectByMouse: true
        background: Rectangle {
            color: "#141419"
            radius: 6
            border.width: hexInput.activeFocus ? 1 : 1
            border.color: hexInput.activeFocus ? "#E1BD22" : "#323240"
        }
        onAccepted: {
            if (/^#[0-9A-Fa-f]{6}$/.test(text)) {
                picker.colorChosen(text.toUpperCase())
            } else {
                text = picker.selectedColor
            }
        }
        onEditingFinished: {
            if (/^#[0-9A-Fa-f]{6}$/.test(text) && text.toUpperCase() !== picker.selectedColor.toUpperCase()) {
                picker.colorChosen(text.toUpperCase())
            }
        }
    }
}
