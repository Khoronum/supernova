import QtQuick
import QtQuick.Layouts

Rectangle {
    id: card
    property int stageNumber: 1
    property int dpiX: 800
    property int dpiY: 800
    property string stageColor: "#E1BD22"
    property bool activeStage: false
    property bool stageEnabled: true

    signal activated()

    implicitHeight: 96
    radius: 10
    color: !stageEnabled ? "#16161B"
         : activeStage ? "#25241D"
         : mouseArea.containsMouse ? "#252530" : "#1C1C24"
    border.width: activeStage ? 2 : 1
    border.color: activeStage ? "#E1BD22" : (mouseArea.containsMouse ? "#454556" : "#2D2D38")
    opacity: stageEnabled ? 1.0 : 0.42

    Behavior on border.color { ColorAnimation { duration: 120 } }
    Behavior on color { ColorAnimation { duration: 120 } }

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: 12
        spacing: 6

        RowLayout {
            Layout.fillWidth: true

            Text {
                text: "STAGE " + card.stageNumber
                font.pixelSize: 11
                font.weight: Font.DemiBold
                font.letterSpacing: 0.8
                color: card.activeStage ? "#E1BD22" : "#9696A4"
            }

            Item { Layout.fillWidth: true }

            Rectangle {
                width: 12
                height: 12
                radius: 6
                color: card.stageColor
                border.width: 1
                border.color: "#55FFFFFF"
            }
        }

        Item { Layout.fillHeight: true }

        Text {
            Layout.fillWidth: true
            text: card.dpiX === card.dpiY ? card.dpiX : (card.dpiX + " × " + card.dpiY)
            font.pixelSize: card.dpiX === card.dpiY ? 22 : 16
            font.weight: Font.Bold
            color: card.activeStage ? "#FFFFFF" : "#E6E6EC"
            elide: Text.ElideRight
        }

        Text {
            text: card.activeStage ? "ACTIVE" : (card.stageEnabled ? "DPI" : "DISABLED")
            font.pixelSize: 10
            font.weight: Font.Medium
            color: card.activeStage ? "#E1BD22" : "#727280"
        }
    }

    MouseArea {
        id: mouseArea
        anchors.fill: parent
        enabled: card.stageEnabled
        hoverEnabled: true
        cursorShape: enabled ? Qt.PointingHandCursor : Qt.ArrowCursor
        onClicked: card.activated()
    }
}
