import QtQuick
import QtQuick.Controls
import com.supernova.ui 1.0

Switch {
    id: control
    required property AppTheme theme

    hoverEnabled: true
    spacing: 10
    leftPadding: 8
    rightPadding: 8
    topPadding: 6
    bottomPadding: 6
    implicitHeight: 38

    background: Rectangle {
        radius: 8
        color: control.enabled && control.hovered ? control.theme.bgControlHover : "transparent"
        Behavior on color { ColorAnimation { duration: 100 } }
    }

    indicator: Rectangle {
        implicitWidth: 42
        implicitHeight: 24
        x: control.leftPadding
        y: parent.height / 2 - height / 2
        radius: 12
        color: control.checked ? control.theme.accent : control.theme.trackOff
        border.width: control.checked ? 0 : 1
        border.color: control.hovered && control.enabled ? control.theme.accent : control.theme.borderControl
        opacity: control.enabled ? 1.0 : 0.45

        Behavior on color { ColorAnimation { duration: 120 } }

        Rectangle {
            width: 18
            height: 18
            radius: 9
            y: (parent.height - height) / 2
            x: control.checked ? parent.width - width - 3 : 3
            color: control.checked ? control.theme.textOnAccent : control.theme.knob

            Behavior on x { NumberAnimation { duration: 120; easing.type: Easing.OutCubic } }
        }
    }

    contentItem: Text {
        text: control.text
        font.pixelSize: 13
        font.weight: Font.Medium
        color: control.enabled ? control.theme.textPrimary : control.theme.textDisabled
        verticalAlignment: Text.AlignVCenter
        leftPadding: control.indicator.width + control.spacing
        elide: Text.ElideRight
    }
}
