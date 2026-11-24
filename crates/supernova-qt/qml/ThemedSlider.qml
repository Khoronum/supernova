import QtQuick
import QtQuick.Controls
import com.supernova.ui 1.0

Slider {
    id: control
    required property AppTheme theme

    hoverEnabled: true
    implicitWidth: 200
    implicitHeight: 26
    opacity: control.enabled ? 1.0 : 0.45

    background: Rectangle {
        x: control.leftPadding
        y: control.topPadding + control.availableHeight / 2 - height / 2
        implicitWidth: 200
        implicitHeight: 6
        width: control.availableWidth
        height: 6
        radius: 3
        color: control.theme.trackOff

        Rectangle {
            width: control.visualPosition * parent.width
            height: parent.height
            radius: 3
            color: control.theme.accent
        }
    }

    handle: Rectangle {
        x: control.leftPadding + control.visualPosition * (control.availableWidth - width)
        y: control.topPadding + control.availableHeight / 2 - height / 2
        implicitWidth: 18
        implicitHeight: 18
        radius: 9
        color: control.pressed ? control.theme.accentPressed
             : control.hovered ? control.theme.accentHover
             : control.theme.accent
        border.color: control.theme.bgCard
        border.width: 2
    }
}
