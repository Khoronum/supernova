import QtQuick
import QtQuick.Controls
import com.supernova.ui 1.0

SpinBox {
    id: control
    required property AppTheme theme

    hoverEnabled: true
    implicitWidth: 128
    implicitHeight: 34
    padding: 0
    leftPadding: 34
    rightPadding: 34
    font.pixelSize: 13
    font.weight: Font.Medium

    background: Rectangle {
        radius: 7
        color: control.enabled ? control.theme.bgInput : control.theme.bgCardDisabled
        border.width: 1
        border.color: (control.activeFocus || (control.contentItem && control.contentItem.activeFocus))
                      ? control.theme.accent
                      : control.theme.borderControl
    }

    contentItem: TextInput {
        z: 2
        text: control.displayText
        font: control.font
        color: control.enabled ? control.theme.textPrimary : control.theme.textDisabled
        selectionColor: control.theme.accent
        selectedTextColor: control.theme.textOnAccent
        horizontalAlignment: Qt.AlignHCenter
        verticalAlignment: Qt.AlignVCenter
        readOnly: !control.editable
        validator: control.validator
        inputMethodHints: control.inputMethodHints
        selectByMouse: true
    }

    down.indicator: Rectangle {
        readonly property bool canStep: control.enabled && control.value > control.from
        x: control.mirrored ? control.width - width - 2 : 2
        y: 2
        width: 30
        height: control.height - 4
        radius: 5
        color: !canStep ? "transparent"
             : control.down.pressed ? control.theme.bgControlPressed
             : control.down.hovered ? control.theme.bgControlHover
             : control.theme.bgControl

        Text {
            anchors.centerIn: parent
            text: "\u2212"
            font.pixelSize: 15
            font.weight: Font.Bold
            color: parent.canStep ? control.theme.textPrimary : control.theme.textDisabled
        }
    }

    up.indicator: Rectangle {
        readonly property bool canStep: control.enabled && control.value < control.to
        x: control.mirrored ? 2 : control.width - width - 2
        y: 2
        width: 30
        height: control.height - 4
        radius: 5
        color: !canStep ? "transparent"
             : control.up.pressed ? control.theme.bgControlPressed
             : control.up.hovered ? control.theme.bgControlHover
             : control.theme.bgControl

        Text {
            anchors.centerIn: parent
            text: "+"
            font.pixelSize: 15
            font.weight: Font.Bold
            color: parent.canStep ? control.theme.textPrimary : control.theme.textDisabled
        }
    }
}
