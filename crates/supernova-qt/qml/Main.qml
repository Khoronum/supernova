import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import com.supernova.ui 1.0

ApplicationWindow {
    id: root
    visible: true
    width: 980
    height: 700
    minimumWidth: 780
    minimumHeight: 560
    color: "#121216"
    title: qsTr("Supernova")

    MouseController {
        id: controller
    }

    Component.onCompleted: controller.refreshDevice()

    Timer {
        interval: 500
        repeat: true
        running: controller.connected && !controller.demoMode
        onTriggered: controller.pollDevice()
    }

    readonly property var parsedStages: {
        try {
            return JSON.parse(controller.stagesJson)
        } catch (e) {
            return []
        }
    }
    readonly property bool controlsReady: controller.connected && controller.mouseOnline
    property int previewDpiX: controller.activeDpiX
    property int previewDpiY: controller.activeDpiY

    Connections {
        target: controller
        function onActiveDpiXChanged() { root.previewDpiX = controller.activeDpiX }
        function onActiveDpiYChanged() { root.previewDpiY = controller.activeDpiY }
    }

    Dialog {
        id: resetDialog
        title: "Restore Factory Defaults"
        modal: true
        anchors.centerIn: Overlay.overlay
        implicitWidth: 440
        standardButtons: Dialog.Yes | Dialog.No
        background: Rectangle {
            color: "#1D1D24"
            radius: 10
            border.color: "#383846"
            border.width: 1
        }
        contentItem: Label {
            text: "Reset all onboard mouse DPI stages, polling rate, and sensor settings to factory defaults?"
            color: "#F4F4F6"
            wrapMode: Text.WordWrap
            padding: 12
        }
        onAccepted: controller.factoryReset()
    }

    ColumnLayout {
        anchors.fill: parent
        spacing: 0

        // Header Bar
        Rectangle {
            Layout.fillWidth: true
            Layout.preferredHeight: 68
            color: "#18181F"

            RowLayout {
                anchors.fill: parent
                anchors.leftMargin: 20
                anchors.rightMargin: 20
                spacing: 12

                // Brand Title
                RowLayout {
                    spacing: 8
                    Rectangle {
                        width: 10
                        height: 26
                        radius: 3
                        color: "#E1BD22"
                    }
                    ColumnLayout {
                        spacing: 0
                        Text {
                            text: "Supernova"
                            color: "#F4F4F6"
                            font.pixelSize: 16
                            font.weight: Font.Bold
                        }
                        Text {
                            text: controller.deviceName
                            color: "#9696A4"
                            font.pixelSize: 12
                        }
                    }
                }

                Item { Layout.fillWidth: true }

                // Status Badges
                Rectangle {
                    visible: controller.connected
                    color: "#23232D"
                    radius: 6
                    border.color: "#343442"
                    border.width: 1
                    implicitHeight: 28
                    implicitWidth: sensorBadgeText.implicitWidth + 18
                    Text {
                        id: sensorBadgeText
                        anchors.centerIn: parent
                        text: controller.sensorName + " · " + controller.connectionMode
                        color: "#D8D8E2"
                        font.pixelSize: 11
                        font.weight: Font.Medium
                    }
                }

                Rectangle {
                    visible: controller.connected && controller.firmwareVersion !== "--"
                    color: "#23232D"
                    radius: 6
                    border.color: "#343442"
                    border.width: 1
                    implicitHeight: 28
                    implicitWidth: fwBadgeText.implicitWidth + 16
                    Text {
                        id: fwBadgeText
                        anchors.centerIn: parent
                        text: "FW " + controller.firmwareVersion
                        color: "#9696A4"
                        font.pixelSize: 11
                    }
                }

                Rectangle {
                    visible: controller.connected && controller.batteryPercent >= 0
                    color: "#1D2620"
                    radius: 6
                    border.color: "#2E4A36"
                    border.width: 1
                    implicitHeight: 28
                    implicitWidth: batText.implicitWidth + 18
                    Text {
                        id: batText
                        anchors.centerIn: parent
                        text: (controller.batteryCharging ? "⚡ " : "") + controller.batteryPercent + "%"
                        color: controller.batteryPercent <= 15 ? "#E74C3C" : "#6EE798"
                        font.pixelSize: 11
                        font.weight: Font.Bold
                    }
                }

                Button {
                    text: "Refresh"
                    implicitHeight: 32
                    onClicked: controller.refreshDevice()
                    contentItem: Text {
                        text: parent.text
                        color: "#F4F4F6"
                        font.pixelSize: 12
                        font.weight: Font.Medium
                        horizontalAlignment: Text.AlignHCenter
                        verticalAlignment: Text.AlignVCenter
                        leftPadding: 10
                        rightPadding: 10
                    }
                    background: Rectangle {
                        radius: 6
                        color: parent.hovered ? "#2F2F3B" : "#24242E"
                        border.color: "#3A3A4A"
                    }
                }

                Button {
                    text: controller.demoMode ? "Exit Demo" : "Demo Mode"
                    implicitHeight: 32
                    onClicked: controller.applyDemoMode(!controller.demoMode)
                    contentItem: Text {
                        text: parent.text
                        color: controller.demoMode ? "#121216" : "#E1BD22"
                        font.pixelSize: 12
                        font.weight: Font.DemiBold
                        horizontalAlignment: Text.AlignHCenter
                        verticalAlignment: Text.AlignVCenter
                        leftPadding: 10
                        rightPadding: 10
                    }
                    background: Rectangle {
                        radius: 6
                        color: controller.demoMode ? "#E1BD22" : (parent.hovered ? "#2E2A1B" : "#222019")
                        border.color: "#E1BD22"
                    }
                }
            }

            Rectangle {
                anchors.bottom: parent.bottom
                width: parent.width
                height: 1
                color: "#2A2A36"
            }
        }

        // Permission / Offline Banner
        Rectangle {
            Layout.fillWidth: true
            visible: !root.controlsReady
            color: controller.permissionDenied ? "#2A1F18" : "#1D1D25"
            implicitHeight: bannerLayout.implicitHeight + 24

            ColumnLayout {
                id: bannerLayout
                anchors.fill: parent
                anchors.margins: 14
                spacing: 8

                RowLayout {
                    Layout.fillWidth: true
                    spacing: 10

                    Text {
                        Layout.fillWidth: true
                        text: controller.errorMessage.length > 0
                              ? controller.errorMessage
                              : controller.statusMessage
                        color: controller.permissionDenied ? "#F6B97A" : "#E0E0E8"
                        font.pixelSize: 13
                        font.weight: Font.Medium
                        wrapMode: Text.WordWrap
                    }

                    Button {
                        visible: !controller.demoMode
                        text: "Try Demo Mode"
                        implicitHeight: 30
                        onClicked: controller.applyDemoMode(true)
                        contentItem: Text {
                            text: parent.text
                            color: "#121216"
                            font.pixelSize: 12
                            font.weight: Font.Bold
                            horizontalAlignment: Text.AlignHCenter
                            verticalAlignment: Text.AlignVCenter
                            leftPadding: 10
                            rightPadding: 10
                        }
                        background: Rectangle {
                            radius: 6
                            color: "#E1BD22"
                        }
                    }
                }

                TextField {
                    visible: controller.permissionDenied
                    Layout.fillWidth: true
                    readOnly: true
                    selectByMouse: true
                    text: controller.udevHelpCommand
                    color: "#F4F4F6"
                    font.family: "monospace"
                    font.pixelSize: 11
                    background: Rectangle {
                        color: "#141418"
                        radius: 6
                        border.color: "#4B3B2B"
                    }
                }
            }

            Rectangle {
                anchors.bottom: parent.bottom
                width: parent.width
                height: 1
                color: "#332B24"
            }
        }

        // Navigation Tabs
        TabBar {
            id: tabBar
            Layout.fillWidth: true
            background: Rectangle {
                color: "#16161C"
                Rectangle {
                    anchors.bottom: parent.bottom
                    width: parent.width
                    height: 1
                    color: "#292934"
                }
            }

            TabButton {
                text: "DPI Settings"
                width: 170
                height: 42
                contentItem: Text {
                    text: parent.text
                    color: tabBar.currentIndex === 0 ? "#E1BD22" : "#9A9AA8"
                    font.pixelSize: 13
                    font.weight: tabBar.currentIndex === 0 ? Font.Bold : Font.Medium
                    horizontalAlignment: Text.AlignHCenter
                    verticalAlignment: Text.AlignVCenter
                }
                background: Rectangle {
                    color: "transparent"
                    Rectangle {
                        visible: tabBar.currentIndex === 0
                        anchors.bottom: parent.bottom
                        width: parent.width
                        height: 2
                        color: "#E1BD22"
                    }
                }
            }

            TabButton {
                text: "Performance & Sensor"
                width: 190
                height: 42
                contentItem: Text {
                    text: parent.text
                    color: tabBar.currentIndex === 1 ? "#E1BD22" : "#9A9AA8"
                    font.pixelSize: 13
                    font.weight: tabBar.currentIndex === 1 ? Font.Bold : Font.Medium
                    horizontalAlignment: Text.AlignHCenter
                    verticalAlignment: Text.AlignVCenter
                }
                background: Rectangle {
                    color: "transparent"
                    Rectangle {
                        visible: tabBar.currentIndex === 1
                        anchors.bottom: parent.bottom
                        width: parent.width
                        height: 2
                        color: "#E1BD22"
                    }
                }
            }
        }

        // Main Content Stack
        StackLayout {
            Layout.fillWidth: true
            Layout.fillHeight: true
            currentIndex: tabBar.currentIndex

            // TAB 1: DPI SETTINGS
            ScrollView {
                clip: true
                contentWidth: availableWidth

                ColumnLayout {
                    width: parent.width
                    spacing: 16

                    Item { implicitHeight: 4 }

                    // Hero Active DPI Card
                    Rectangle {
                        Layout.fillWidth: true
                        Layout.leftMargin: 20
                        Layout.rightMargin: 20
                        implicitHeight: heroRow.implicitHeight + 28
                        radius: 12
                        color: "#1B1B22"
                        border.color: "#2E2E3A"
                        border.width: 1

                        RowLayout {
                            id: heroRow
                            anchors.fill: parent
                            anchors.margins: 16
                            spacing: 18

                            Rectangle {
                                width: 10
                                Layout.fillHeight: true
                                radius: 5
                                color: controller.activeColor
                            }

                            ColumnLayout {
                                spacing: 2
                                Text {
                                    text: "ACTIVE SENSITIVITY · STAGE " + (controller.activeStage + 1)
                                    color: "#9696A4"
                                    font.pixelSize: 11
                                    font.weight: Font.DemiBold
                                    font.letterSpacing: 0.8
                                }
                                Text {
                                    text: controller.xyLinked || !controller.supportsXy
                                          ? (root.previewDpiX + " DPI")
                                          : (root.previewDpiX + " × " + root.previewDpiY + " DPI")
                                    color: "#FFFFFF"
                                    font.pixelSize: 34
                                    font.weight: Font.Bold
                                }
                            }

                            Item { Layout.fillWidth: true }

                            ColumnLayout {
                                spacing: 4
                                Text {
                                    text: "Enabled Stages"
                                    color: "#9696A4"
                                    font.pixelSize: 11
                                    Layout.alignment: Qt.AlignRight
                                }
                                RowLayout {
                                    spacing: 6
                                    Repeater {
                                        model: controller.maxStages
                                        delegate: Button {
                                            required property int index
                                            readonly property int countVal: index + 1
                                            implicitWidth: 32
                                            implicitHeight: 30
                                            enabled: root.controlsReady
                                            text: countVal.toString()
                                            onClicked: controller.applyStageCount(countVal)
                                            contentItem: Text {
                                                text: parent.text
                                                color: controller.stageCount === countVal ? "#121216" : "#E4E4EC"
                                                font.pixelSize: 12
                                                font.weight: Font.Bold
                                                horizontalAlignment: Text.AlignHCenter
                                                verticalAlignment: Text.AlignVCenter
                                            }
                                            background: Rectangle {
                                                radius: 6
                                                color: controller.stageCount === countVal ? "#E1BD22" : "#25252F"
                                                border.color: controller.stageCount === countVal ? "#E1BD22" : "#353545"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // DPI Stages Grid
                    RowLayout {
                        Layout.fillWidth: true
                        Layout.leftMargin: 20
                        Layout.rightMargin: 20
                        spacing: 10

                        Repeater {
                            model: root.parsedStages
                            delegate: DpiStageCard {
                                required property var modelData
                                Layout.fillWidth: true
                                stageNumber: modelData.stage
                                dpiX: modelData.dpiX
                                dpiY: modelData.dpiY
                                stageColor: modelData.color
                                activeStage: modelData.active
                                stageEnabled: modelData.enabled && root.controlsReady
                                onActivated: controller.selectStage(modelData.index)
                            }
                        }
                    }

                    // Selected Stage DPI Editor Card
                    Rectangle {
                        Layout.fillWidth: true
                        Layout.leftMargin: 20
                        Layout.rightMargin: 20
                        implicitHeight: editorCol.implicitHeight + 32
                        radius: 12
                        color: "#1B1B22"
                        border.color: "#2E2E3A"
                        border.width: 1

                        ColumnLayout {
                            id: editorCol
                            anchors.fill: parent
                            anchors.margins: 16
                            spacing: 14

                            RowLayout {
                                Layout.fillWidth: true
                                Text {
                                    text: "Stage " + (controller.activeStage + 1) + " Tuning"
                                    color: "#F4F4F6"
                                    font.pixelSize: 15
                                    font.weight: Font.Bold
                                }
                                Item { Layout.fillWidth: true }
                                CheckBox {
                                    visible: controller.supportsXy
                                    enabled: root.controlsReady
                                    checked: !controller.xyLinked
                                    text: "Independent X / Y DPI"
                                    onToggled: controller.applyXyLinked(!checked)
                                    contentItem: Text {
                                        text: parent.text
                                        color: "#D0D0DA"
                                        font.pixelSize: 12
                                        leftPadding: parent.indicator.width + 6
                                        verticalAlignment: Text.AlignVCenter
                                    }
                                    indicator: Rectangle {
                                        implicitWidth: 16
                                        implicitHeight: 16
                                        x: parent.leftPadding
                                        y: parent.height / 2 - height / 2
                                        radius: 4
                                        color: parent.checked ? "#E1BD22" : "#141419"
                                        border.color: parent.checked ? "#E1BD22" : "#4A4A5A"
                                    }
                                }
                            }

                            // Quick Presets
                            RowLayout {
                                Layout.fillWidth: true
                                spacing: 8
                                Text {
                                    text: "Presets:"
                                    color: "#9696A4"
                                    font.pixelSize: 12
                                }
                                Repeater {
                                    model: [400, 800, 1000, 1200, 1600, 2400, 3200, 6400]
                                    delegate: Button {
                                        required property int modelData
                                        visible: modelData >= controller.dpiMin && modelData <= controller.dpiMax
                                        enabled: root.controlsReady
                                        implicitHeight: 28
                                        text: modelData.toString()
                                        onClicked: controller.setStageDpi(
                                            controller.activeStage,
                                            modelData,
                                            controller.xyLinked ? modelData : controller.activeDpiY
                                        )
                                        contentItem: Text {
                                            text: parent.text
                                            color: controller.activeDpiX === modelData ? "#121216" : "#DCDCE6"
                                            font.pixelSize: 11
                                            font.weight: Font.DemiBold
                                            leftPadding: 10
                                            rightPadding: 10
                                            verticalAlignment: Text.AlignVCenter
                                            horizontalAlignment: Text.AlignHCenter
                                        }
                                        background: Rectangle {
                                            radius: 6
                                            color: controller.activeDpiX === modelData ? "#E1BD22" : "#25252F"
                                            border.color: controller.activeDpiX === modelData ? "#E1BD22" : "#363646"
                                        }
                                    }
                                }
                                Item { Layout.fillWidth: true }
                            }

                            // X (or Unified) DPI Slider + SpinBox
                            RowLayout {
                                Layout.fillWidth: true
                                spacing: 12

                                Text {
                                    text: controller.xyLinked || !controller.supportsXy ? "DPI" : "X DPI"
                                    color: "#C4C4D0"
                                    font.pixelSize: 13
                                    font.weight: Font.Medium
                                    Layout.preferredWidth: 46
                                }

                                Slider {
                                    id: dpiXSlider
                                    Layout.fillWidth: true
                                    enabled: root.controlsReady
                                    from: controller.dpiMin
                                    to: controller.dpiMax
                                    stepSize: controller.dpiStep
                                    snapMode: Slider.SnapAlways
                                    value: root.previewDpiX
                                    onMoved: root.previewDpiX = Math.round(value)
                                    onPressedChanged: {
                                        if (!pressed && root.controlsReady) {
                                            let target = Math.round(value)
                                            controller.setStageDpi(
                                                controller.activeStage,
                                                target,
                                                controller.xyLinked ? target : root.previewDpiY
                                            )
                                        }
                                    }
                                    background: Rectangle {
                                        x: dpiXSlider.leftPadding
                                        y: dpiXSlider.topPadding + dpiXSlider.availableHeight / 2 - height / 2
                                        implicitWidth: 200
                                        implicitHeight: 6
                                        width: dpiXSlider.availableWidth
                                        height: 6
                                        radius: 3
                                        color: "#2C2C38"
                                        Rectangle {
                                            width: dpiXSlider.visualPosition * parent.width
                                            height: parent.height
                                            radius: 3
                                            color: "#E1BD22"
                                        }
                                    }
                                    handle: Rectangle {
                                        x: dpiXSlider.leftPadding + dpiXSlider.visualPosition * (dpiXSlider.availableWidth - width)
                                        y: dpiXSlider.topPadding + dpiXSlider.availableHeight / 2 - height / 2
                                        implicitWidth: 18
                                        implicitHeight: 18
                                        radius: 9
                                        color: dpiXSlider.pressed ? "#FFF1A8" : "#E1BD22"
                                        border.color: "#121216"
                                        border.width: 2
                                    }
                                }

                                SpinBox {
                                    id: dpiXSpin
                                    enabled: root.controlsReady
                                    from: controller.dpiMin
                                    to: controller.dpiMax
                                    stepSize: controller.dpiStep
                                    editable: true
                                    value: root.previewDpiX
                                    Layout.preferredWidth: 120
                                    onValueModified: {
                                        root.previewDpiX = value
                                        controller.setStageDpi(
                                            controller.activeStage,
                                            value,
                                            controller.xyLinked ? value : root.previewDpiY
                                        )
                                    }
                                }
                            }

                            // Y DPI Slider + SpinBox (when X/Y unlinked)
                            RowLayout {
                                Layout.fillWidth: true
                                visible: controller.supportsXy && !controller.xyLinked
                                spacing: 12

                                Text {
                                    text: "Y DPI"
                                    color: "#C4C4D0"
                                    font.pixelSize: 13
                                    font.weight: Font.Medium
                                    Layout.preferredWidth: 46
                                }

                                Slider {
                                    id: dpiYSlider
                                    Layout.fillWidth: true
                                    enabled: root.controlsReady
                                    from: controller.dpiMin
                                    to: controller.dpiMax
                                    stepSize: controller.dpiStep
                                    snapMode: Slider.SnapAlways
                                    value: root.previewDpiY
                                    onMoved: root.previewDpiY = Math.round(value)
                                    onPressedChanged: {
                                        if (!pressed && root.controlsReady) {
                                            controller.setStageDpi(
                                                controller.activeStage,
                                                root.previewDpiX,
                                                Math.round(value)
                                            )
                                        }
                                    }
                                    background: Rectangle {
                                        x: dpiYSlider.leftPadding
                                        y: dpiYSlider.topPadding + dpiYSlider.availableHeight / 2 - height / 2
                                        implicitWidth: 200
                                        implicitHeight: 6
                                        width: dpiYSlider.availableWidth
                                        height: 6
                                        radius: 3
                                        color: "#2C2C38"
                                        Rectangle {
                                            width: dpiYSlider.visualPosition * parent.width
                                            height: parent.height
                                            radius: 3
                                            color: "#E1BD22"
                                        }
                                    }
                                    handle: Rectangle {
                                        x: dpiYSlider.leftPadding + dpiYSlider.visualPosition * (dpiYSlider.availableWidth - width)
                                        y: dpiYSlider.topPadding + dpiYSlider.availableHeight / 2 - height / 2
                                        implicitWidth: 18
                                        implicitHeight: 18
                                        radius: 9
                                        color: dpiYSlider.pressed ? "#FFF1A8" : "#E1BD22"
                                        border.color: "#121216"
                                        border.width: 2
                                    }
                                }

                                SpinBox {
                                    enabled: root.controlsReady
                                    from: controller.dpiMin
                                    to: controller.dpiMax
                                    stepSize: controller.dpiStep
                                    editable: true
                                    value: root.previewDpiY
                                    Layout.preferredWidth: 120
                                    onValueModified: {
                                        root.previewDpiY = value
                                        controller.setStageDpi(controller.activeStage, root.previewDpiX, value)
                                    }
                                }
                            }

                            // Stage LED Color Row
                            RowLayout {
                                Layout.fillWidth: true
                                spacing: 12
                                Text {
                                    text: "Stage Color:"
                                    color: "#9696A4"
                                    font.pixelSize: 12
                                }
                                ColorSwatchPicker {
                                    controlsEnabled: root.controlsReady
                                    selectedColor: controller.activeColor
                                    onColorChosen: function(hex) {
                                        controller.setStageColor(controller.activeStage, hex)
                                    }
                                }
                                Item { Layout.fillWidth: true }
                            }
                        }
                    }

                    // DPI Indicator LED Card
                    Rectangle {
                        Layout.fillWidth: true
                        Layout.leftMargin: 20
                        Layout.rightMargin: 20
                        implicitHeight: ledCol.implicitHeight + 28
                        radius: 12
                        color: "#1B1B22"
                        border.color: "#2E2E3A"
                        border.width: 1

                        ColumnLayout {
                            id: ledCol
                            anchors.fill: parent
                            anchors.margins: 16
                            spacing: 12

                            RowLayout {
                                Layout.fillWidth: true
                                Text {
                                    text: "DPI Indicator LED"
                                    color: "#F4F4F6"
                                    font.pixelSize: 14
                                    font.weight: Font.Bold
                                }
                                Item { Layout.fillWidth: true }
                                Repeater {
                                    model: [
                                        { id: 0, label: "Off" },
                                        { id: 1, label: "Static" },
                                        { id: 2, label: "Breathing" }
                                    ]
                                    delegate: Button {
                                        required property var modelData
                                        enabled: root.controlsReady
                                        implicitHeight: 28
                                        text: modelData.label
                                        onClicked: controller.setDpiLed(
                                            modelData.id,
                                            controller.dpiLedBrightness,
                                            controller.dpiLedSpeed
                                        )
                                        contentItem: Text {
                                            text: parent.text
                                            color: controller.dpiLedMode === modelData.id ? "#121216" : "#DCDCE6"
                                            font.pixelSize: 12
                                            font.weight: Font.DemiBold
                                            leftPadding: 12
                                            rightPadding: 12
                                            verticalAlignment: Text.AlignVCenter
                                            horizontalAlignment: Text.AlignHCenter
                                        }
                                        background: Rectangle {
                                            radius: 6
                                            color: controller.dpiLedMode === modelData.id ? "#E1BD22" : "#25252F"
                                            border.color: controller.dpiLedMode === modelData.id ? "#E1BD22" : "#363646"
                                        }
                                    }
                                }
                            }

                            RowLayout {
                                Layout.fillWidth: true
                                spacing: 14
                                Text {
                                    text: "Brightness"
                                    color: "#9696A4"
                                    font.pixelSize: 12
                                    Layout.preferredWidth: 72
                                }
                                Slider {
                                    Layout.fillWidth: true
                                    enabled: root.controlsReady && controller.dpiLedMode !== 0
                                    from: 0
                                    to: 255
                                    stepSize: 16
                                    value: controller.dpiLedBrightness
                                    onPressedChanged: {
                                        if (!pressed && root.controlsReady) {
                                            controller.setDpiLed(
                                                controller.dpiLedMode,
                                                Math.round(value),
                                                controller.dpiLedSpeed
                                            )
                                        }
                                    }
                                }
                                Text {
                                    text: "Speed"
                                    color: "#9696A4"
                                    font.pixelSize: 12
                                }
                                Slider {
                                    Layout.preferredWidth: 140
                                    enabled: root.controlsReady && controller.dpiLedMode === 2
                                    from: 1
                                    to: 5
                                    stepSize: 1
                                    snapMode: Slider.SnapAlways
                                    value: controller.dpiLedSpeed
                                    onPressedChanged: {
                                        if (!pressed && root.controlsReady) {
                                            controller.setDpiLed(
                                                controller.dpiLedMode,
                                                controller.dpiLedBrightness,
                                                Math.round(value)
                                            )
                                        }
                                    }
                                }
                            }
                        }
                    }

                    Item { implicitHeight: 12 }
                }
            }

            // TAB 2: PERFORMANCE & SENSOR
            ScrollView {
                clip: true
                contentWidth: availableWidth

                ColumnLayout {
                    width: parent.width
                    spacing: 16

                    Item { implicitHeight: 8 }

                    // Polling Rate Card
                    Rectangle {
                        Layout.fillWidth: true
                        Layout.leftMargin: 20
                        Layout.rightMargin: 20
                        implicitHeight: rateCol.implicitHeight + 28
                        radius: 12
                        color: "#1B1B22"
                        border.color: "#2E2E3A"
                        border.width: 1

                        ColumnLayout {
                            id: rateCol
                            anchors.fill: parent
                            anchors.margins: 16
                            spacing: 12

                            Text {
                                text: "USB Polling Rate"
                                color: "#F4F4F6"
                                font.pixelSize: 15
                                font.weight: Font.Bold
                            }

                            RowLayout {
                                Layout.fillWidth: true
                                spacing: 8
                                Repeater {
                                    model: [125, 250, 500, 1000, 2000, 4000, 8000]
                                    delegate: Button {
                                        required property int modelData
                                        Layout.fillWidth: true
                                        implicitHeight: 36
                                        enabled: root.controlsReady
                                        text: modelData >= 1000 ? ((modelData / 1000) + "000 Hz") : (modelData + " Hz")
                                        onClicked: controller.setPollingRate(modelData)
                                        contentItem: Text {
                                            text: parent.text
                                            color: controller.pollingRateHz === modelData ? "#121216" : "#E4E4EC"
                                            font.pixelSize: 12
                                            font.weight: Font.Bold
                                            horizontalAlignment: Text.AlignHCenter
                                            verticalAlignment: Text.AlignVCenter
                                        }
                                        background: Rectangle {
                                            radius: 8
                                            color: controller.pollingRateHz === modelData ? "#E1BD22" : "#25252F"
                                            border.color: controller.pollingRateHz === modelData ? "#E1BD22" : "#353545"
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Sensor Tuning & LOD Card
                    Rectangle {
                        Layout.fillWidth: true
                        Layout.leftMargin: 20
                        Layout.rightMargin: 20
                        implicitHeight: sensorCol.implicitHeight + 28
                        radius: 12
                        color: "#1B1B22"
                        border.color: "#2E2E3A"
                        border.width: 1

                        ColumnLayout {
                            id: sensorCol
                            anchors.fill: parent
                            anchors.margins: 16
                            spacing: 14

                            Text {
                                text: "Sensor Features & Lift-Off Distance"
                                color: "#F4F4F6"
                                font.pixelSize: 15
                                font.weight: Font.Bold
                            }

                            RowLayout {
                                Layout.fillWidth: true
                                spacing: 10
                                Text {
                                    text: "Lift-Off Distance (LOD):"
                                    color: "#B0B0BC"
                                    font.pixelSize: 13
                                }
                                Repeater {
                                    model: [
                                        { code: 3, label: "0.7 mm", needSubMm: true },
                                        { code: 1, label: "1.0 mm", needSubMm: false },
                                        { code: 2, label: "2.0 mm", needSubMm: false }
                                    ]
                                    delegate: Button {
                                        required property var modelData
                                        visible: !modelData.needSubMm || controller.supportsSubMmLod
                                        enabled: root.controlsReady
                                        implicitHeight: 30
                                        text: modelData.label
                                        onClicked: controller.setLod(modelData.code)
                                        contentItem: Text {
                                            text: parent.text
                                            color: controller.lodLevel === modelData.code ? "#121216" : "#E4E4EC"
                                            font.pixelSize: 12
                                            font.weight: Font.Bold
                                            leftPadding: 14
                                            rightPadding: 14
                                            horizontalAlignment: Text.AlignHCenter
                                            verticalAlignment: Text.AlignVCenter
                                        }
                                        background: Rectangle {
                                            radius: 6
                                            color: controller.lodLevel === modelData.code ? "#E1BD22" : "#25252F"
                                            border.color: controller.lodLevel === modelData.code ? "#E1BD22" : "#353545"
                                        }
                                    }
                                }
                                Item { Layout.fillWidth: true }
                            }

                            GridLayout {
                                Layout.fillWidth: true
                                columns: 2
                                columnSpacing: 20
                                rowSpacing: 10

                                Switch {
                                    enabled: root.controlsReady
                                    checked: controller.motionSync
                                    text: "Motion Sync"
                                    onToggled: controller.applyMotionSync(checked)
                                }
                                Switch {
                                    enabled: root.controlsReady
                                    checked: controller.angleSnapping
                                    text: "Angle Snapping"
                                    onToggled: controller.applyAngleSnapping(checked)
                                }
                                Switch {
                                    enabled: root.controlsReady
                                    checked: controller.rippleControl
                                    text: "Ripple Control"
                                    onToggled: controller.applyRippleControl(checked)
                                }
                                Switch {
                                    enabled: root.controlsReady
                                    checked: controller.highPerformance
                                    text: "High Performance Mode"
                                    onToggled: controller.applyHighPerformance(checked)
                                }
                            }
                        }
                    }

                    // Debounce, Sleep, and Reset Card
                    Rectangle {
                        Layout.fillWidth: true
                        Layout.leftMargin: 20
                        Layout.rightMargin: 20
                        implicitHeight: timingCol.implicitHeight + 28
                        radius: 12
                        color: "#1B1B22"
                        border.color: "#2E2E3A"
                        border.width: 1

                        ColumnLayout {
                            id: timingCol
                            anchors.fill: parent
                            anchors.margins: 16
                            spacing: 12

                            Text {
                                text: "Button Debounce, Sleep & Factory Reset"
                                color: "#F4F4F6"
                                font.pixelSize: 15
                                font.weight: Font.Bold
                            }

                            RowLayout {
                                Layout.fillWidth: true
                                spacing: 12
                                Text {
                                    text: "Debounce Time:"
                                    color: "#B0B0BC"
                                    font.pixelSize: 13
                                    Layout.preferredWidth: 115
                                }
                                SpinBox {
                                    enabled: root.controlsReady
                                    from: 0
                                    to: 30
                                    value: controller.debounceMs
                                    onValueModified: controller.applyDebounceMs(value)
                                }
                                Text {
                                    text: controller.debounceMs < 3
                                          ? "ms (⚠ < 3 ms may cause double-clicking)"
                                          : "ms"
                                    color: controller.debounceMs < 3 ? "#F6B97A" : "#9696A4"
                                    font.pixelSize: 12
                                }
                                Item { Layout.fillWidth: true }
                            }

                            RowLayout {
                                Layout.fillWidth: true
                                spacing: 12
                                Text {
                                    text: "Sleep Timeout:"
                                    color: "#B0B0BC"
                                    font.pixelSize: 13
                                    Layout.preferredWidth: 115
                                }
                                SpinBox {
                                    enabled: root.controlsReady
                                    from: 1
                                    to: 40
                                    value: controller.sleepMinutes
                                    onValueModified: controller.applySleepMinutes(value)
                                }
                                Text {
                                    text: "minutes"
                                    color: "#9696A4"
                                    font.pixelSize: 12
                                }
                                Item { Layout.fillWidth: true }

                                Button {
                                    enabled: root.controlsReady
                                    text: "Factory Reset"
                                    implicitHeight: 34
                                    onClicked: resetDialog.open()
                                    contentItem: Text {
                                        text: parent.text
                                        color: "#FF8A80"
                                        font.pixelSize: 12
                                        font.weight: Font.Bold
                                        leftPadding: 14
                                        rightPadding: 14
                                        horizontalAlignment: Text.AlignHCenter
                                        verticalAlignment: Text.AlignVCenter
                                    }
                                    background: Rectangle {
                                        radius: 6
                                        color: "#2C1B1E"
                                        border.color: "#7A343B"
                                    }
                                }
                            }
                        }
                    }

                    Item { implicitHeight: 12 }
                }
            }
        }

        // Footer Status Bar
        Rectangle {
            Layout.fillWidth: true
            Layout.preferredHeight: 30
            color: "#16161C"
            Rectangle {
                anchors.top: parent.top
                width: parent.width
                height: 1
                color: "#292934"
            }
            RowLayout {
                anchors.fill: parent
                anchors.leftMargin: 16
                anchors.rightMargin: 16
                Text {
                    Layout.fillWidth: true
                    text: controller.errorMessage.length > 0 ? controller.errorMessage : controller.statusMessage
                    color: controller.errorMessage.length > 0 ? "#F6B97A" : "#8F8F9D"
                    font.pixelSize: 11
                    elide: Text.ElideRight
                }
                Text {
                    text: "VID 0x379A · Report 0x08"
                    color: "#60606D"
                    font.pixelSize: 11
                }
            }
        }
    }
}
