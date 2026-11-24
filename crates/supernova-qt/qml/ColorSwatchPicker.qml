import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import com.supernova.ui 1.0

Flow {
    id: picker
    required property AppTheme theme
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
            readonly property bool selected: picker.selectedColor.toUpperCase() === modelData
            width: 26
            height: 26
            radius: 13
            color: modelData
            border.width: selected ? 3 : 1
            border.color: selected ? picker.theme.textPrimary : picker.theme.borderControl
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
        width: 96
        height: 28
        enabled: picker.controlsEnabled
        text: picker.selectedColor
        color: picker.enabled ? picker.theme.textPrimary : picker.theme.textDisabled
        selectionColor: picker.theme.accent
        selectedTextColor: picker.theme.textOnAccent
        font.family: "monospace"
        font.pixelSize: 12
        horizontalAlignment: Text.AlignHCenter
        verticalAlignment: Text.AlignVCenter
        maximumLength: 7
        selectByMouse: true
        background: Rectangle {
            color: picker.enabled ? picker.theme.bgInput : picker.theme.bgCardDisabled
            radius: 6
            border.width: 1
            border.color: hexInput.activeFocus ? picker.theme.accent : picker.theme.borderControl
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
