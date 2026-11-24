import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import com.supernova.ui 1.0

ColumnLayout {
    id: dpiPane
    required property var controller
    required property AppTheme theme
    required property bool controlsReady
    property bool showSectionHeader: false

    readonly property var parsedStages: {
        try {
            return JSON.parse(controller.stagesJson)
        } catch (e) {
            return []
        }
    }
    property int previewDpiX: controller.activeDpiX
    property int previewDpiY: controller.activeDpiY

    Connections {
        target: dpiPane.controller
        function onActiveDpiXChanged() { dpiPane.previewDpiX = dpiPane.controller.activeDpiX }
        function onActiveDpiYChanged() { dpiPane.previewDpiY = dpiPane.controller.activeDpiY }
    }

    spacing: 14

    Text {
        visible: dpiPane.showSectionHeader
        text: "DPI SETTINGS"
        color: dpiPane.theme.textMuted
        font.pixelSize: 11
        font.weight: Font.Bold
        font.letterSpacing: 1.0
    }

    // Hero Active DPI Card
    Rectangle {
        Layout.fillWidth: true
        implicitHeight: heroGrid.implicitHeight + 32
        radius: 12
        color: dpiPane.theme.bgCard
        border.color: dpiPane.theme.border
        border.width: 1

        GridLayout {
            id: heroGrid
            anchors.fill: parent
            anchors.margins: 16
            columns: dpiPane.width >= 480 ? 2 : 1
            columnSpacing: 16
            rowSpacing: 12

            RowLayout {
                Layout.fillWidth: true
                spacing: 14

                Rectangle {
                    width: 8
                    Layout.preferredHeight: 54
                    radius: 4
                    color: dpiPane.controller.activeColor
                    border.width: 1
                    border.color: dpiPane.theme.borderControl
                }

                ColumnLayout {
                    spacing: 2
                    Text {
                        text: "ACTIVE SENSITIVITY · STAGE " + (dpiPane.controller.activeStage + 1)
                        color: dpiPane.theme.textMuted
                        font.pixelSize: 11
                        font.weight: Font.DemiBold
                        font.letterSpacing: 0.8
                    }
                    Text {
                        text: dpiPane.controller.xyLinked || !dpiPane.controller.supportsXy
                              ? (dpiPane.previewDpiX + " DPI")
                              : (dpiPane.previewDpiX + " × " + dpiPane.previewDpiY + " DPI")
                        color: dpiPane.theme.textPrimary
                        font.pixelSize: dpiPane.width >= 540 ? 34 : 28
                        font.weight: Font.Bold
                    }
                }
            }

            ColumnLayout {
                spacing: 6
                Layout.alignment: dpiPane.width >= 480 ? (Qt.AlignRight | Qt.AlignVCenter) : Qt.AlignLeft

                Text {
                    text: "Enabled Stages"
                    color: dpiPane.theme.textMuted
                    font.pixelSize: 11
                    Layout.alignment: dpiPane.width >= 480 ? Qt.AlignRight : Qt.AlignLeft
                }

                RowLayout {
                    spacing: 6
                    Repeater {
                        model: dpiPane.controller.maxStages
                        delegate: Button {
                            required property int index
                            readonly property int countVal: index + 1
                            readonly property bool selected: dpiPane.controller.stageCount === countVal
                            implicitWidth: 32
                            implicitHeight: 30
                            hoverEnabled: true
                            enabled: dpiPane.controlsReady
                            text: countVal.toString()
                            onClicked: dpiPane.controller.applyStageCount(countVal)
                            contentItem: Text {
                                text: parent.text
                                color: parent.selected ? dpiPane.theme.textOnAccent
                                     : parent.enabled ? dpiPane.theme.textPrimary : dpiPane.theme.textDisabled
                                font.pixelSize: 12
                                font.weight: Font.Bold
                                horizontalAlignment: Text.AlignHCenter
                                verticalAlignment: Text.AlignVCenter
                            }
                            background: Rectangle {
                                radius: 6
                                color: parent.selected ? dpiPane.theme.accent
                                     : parent.hovered && parent.enabled ? dpiPane.theme.bgControlHover
                                     : dpiPane.theme.bgControl
                                border.color: parent.selected ? dpiPane.theme.accent : dpiPane.theme.borderControl
                            }
                        }
                    }
                }
            }
        }
    }

    // DPI Stages Grid
    GridLayout {
        Layout.fillWidth: true
        columnSpacing: 10
        rowSpacing: 10
        columns: dpiPane.width >= 660
                 ? Math.max(1, Math.min(6, dpiPane.parsedStages.length))
                 : (dpiPane.width >= 420 ? 3 : 2)

        Repeater {
            model: dpiPane.parsedStages
            delegate: DpiStageCard {
                required property var modelData
                Layout.fillWidth: true
                theme: dpiPane.theme
                stageNumber: modelData.stage
                dpiX: modelData.dpiX
                dpiY: modelData.dpiY
                stageColor: modelData.color
                activeStage: modelData.active
                stageEnabled: modelData.enabled && dpiPane.controlsReady
                onActivated: dpiPane.controller.selectStage(modelData.index)
            }
        }
    }

    // Selected Stage DPI Editor Card
    Rectangle {
        Layout.fillWidth: true
        implicitHeight: editorCol.implicitHeight + 32
        radius: 12
        color: dpiPane.theme.bgCard
        border.color: dpiPane.theme.border
        border.width: 1

        ColumnLayout {
            id: editorCol
            anchors.fill: parent
            anchors.margins: 16
            spacing: 14

            RowLayout {
                Layout.fillWidth: true
                spacing: 10

                Text {
                    text: "Stage " + (dpiPane.controller.activeStage + 1) + " Tuning"
                    color: dpiPane.theme.textPrimary
                    font.pixelSize: 15
                    font.weight: Font.Bold
                }

                Item { Layout.fillWidth: true }

                CheckBox {
                    visible: dpiPane.controller.supportsXy
                    enabled: dpiPane.controlsReady
                    hoverEnabled: true
                    checked: !dpiPane.controller.xyLinked
                    text: "Independent X / Y DPI"
                    onToggled: dpiPane.controller.applyXyLinked(!checked)
                    contentItem: Text {
                        text: parent.text
                        color: parent.enabled ? dpiPane.theme.textSecondary : dpiPane.theme.textDisabled
                        font.pixelSize: 12
                        font.weight: Font.Medium
                        leftPadding: parent.indicator.width + 8
                        verticalAlignment: Text.AlignVCenter
                    }
                    indicator: Rectangle {
                        implicitWidth: 18
                        implicitHeight: 18
                        x: parent.leftPadding
                        y: parent.height / 2 - height / 2
                        radius: 4
                        color: parent.checked ? dpiPane.theme.accent : dpiPane.theme.bgInput
                        border.width: 1
                        border.color: parent.checked ? dpiPane.theme.accent : dpiPane.theme.borderControl

                        Text {
                            visible: parent.parent.checked
                            anchors.centerIn: parent
                            text: "\u2713"
                            color: dpiPane.theme.textOnAccent
                            font.pixelSize: 12
                            font.weight: Font.Bold
                        }
                    }
                }
            }

            // Quick Presets
            RowLayout {
                Layout.fillWidth: true
                spacing: 10

                Text {
                    text: "Presets:"
                    color: dpiPane.theme.textMuted
                    font.pixelSize: 12
                    topPadding: 5
                    Layout.alignment: Qt.AlignTop | Qt.AlignLeft
                }

                Flow {
                    Layout.fillWidth: true
                    spacing: 8

                    Repeater {
                        model: [400, 800, 1000, 1200, 1600, 2400, 3200, 6400]
                        delegate: Button {
                            required property int modelData
                            readonly property bool selected: dpiPane.controller.activeDpiX === modelData
                            visible: modelData >= dpiPane.controller.dpiMin && modelData <= dpiPane.controller.dpiMax
                            enabled: dpiPane.controlsReady
                            hoverEnabled: true
                            implicitHeight: 28
                            text: modelData.toString()
                            onClicked: dpiPane.controller.setStageDpi(
                                dpiPane.controller.activeStage,
                                modelData,
                                dpiPane.controller.xyLinked ? modelData : dpiPane.controller.activeDpiY
                            )
                            contentItem: Text {
                                text: parent.text
                                color: parent.selected ? dpiPane.theme.textOnAccent
                                     : parent.enabled ? dpiPane.theme.textPrimary : dpiPane.theme.textDisabled
                                font.pixelSize: 11
                                font.weight: Font.DemiBold
                                leftPadding: 10
                                rightPadding: 10
                                verticalAlignment: Text.AlignVCenter
                                horizontalAlignment: Text.AlignHCenter
                            }
                            background: Rectangle {
                                radius: 6
                                color: parent.selected ? dpiPane.theme.accent
                                     : parent.hovered && parent.enabled ? dpiPane.theme.bgControlHover
                                     : dpiPane.theme.bgControl
                                border.color: parent.selected ? dpiPane.theme.accent : dpiPane.theme.borderControl
                            }
                        }
                    }
                }
            }

            // X (or Unified) DPI Slider + SpinBox
            RowLayout {
                Layout.fillWidth: true
                spacing: 12

                Text {
                    text: dpiPane.controller.xyLinked || !dpiPane.controller.supportsXy ? "DPI" : "X DPI"
                    color: dpiPane.theme.textSecondary
                    font.pixelSize: 13
                    font.weight: Font.Medium
                    Layout.preferredWidth: 46
                }

                ThemedSlider {
                    id: dpiXSlider
                    theme: dpiPane.theme
                    Layout.fillWidth: true
                    enabled: dpiPane.controlsReady
                    from: dpiPane.controller.dpiMin
                    to: dpiPane.controller.dpiMax
                    stepSize: dpiPane.controller.dpiStep
                    snapMode: Slider.SnapAlways
                    value: dpiPane.previewDpiX
                    onMoved: dpiPane.previewDpiX = Math.round(value)
                    onPressedChanged: {
                        if (!pressed && dpiPane.controlsReady) {
                            let target = Math.round(value)
                            dpiPane.controller.setStageDpi(
                                dpiPane.controller.activeStage,
                                target,
                                dpiPane.controller.xyLinked ? target : dpiPane.previewDpiY
                            )
                        }
                    }
                }

                ThemedSpinBox {
                    id: dpiXSpin
                    theme: dpiPane.theme
                    enabled: dpiPane.controlsReady
                    from: dpiPane.controller.dpiMin
                    to: dpiPane.controller.dpiMax
                    stepSize: dpiPane.controller.dpiStep
                    editable: true
                    value: dpiPane.previewDpiX
                    Layout.preferredWidth: 128
                    onValueModified: {
                        dpiPane.previewDpiX = value
                        dpiPane.controller.setStageDpi(
                            dpiPane.controller.activeStage,
                            value,
                            dpiPane.controller.xyLinked ? value : dpiPane.previewDpiY
                        )
                    }
                }
            }

            // Y DPI Slider + SpinBox (when X/Y unlinked)
            RowLayout {
                Layout.fillWidth: true
                visible: dpiPane.controller.supportsXy && !dpiPane.controller.xyLinked
                spacing: 12

                Text {
                    text: "Y DPI"
                    color: dpiPane.theme.textSecondary
                    font.pixelSize: 13
                    font.weight: Font.Medium
                    Layout.preferredWidth: 46
                }

                ThemedSlider {
                    id: dpiYSlider
                    theme: dpiPane.theme
                    Layout.fillWidth: true
                    enabled: dpiPane.controlsReady
                    from: dpiPane.controller.dpiMin
                    to: dpiPane.controller.dpiMax
                    stepSize: dpiPane.controller.dpiStep
                    snapMode: Slider.SnapAlways
                    value: dpiPane.previewDpiY
                    onMoved: dpiPane.previewDpiY = Math.round(value)
                    onPressedChanged: {
                        if (!pressed && dpiPane.controlsReady) {
                            dpiPane.controller.setStageDpi(
                                dpiPane.controller.activeStage,
                                dpiPane.previewDpiX,
                                Math.round(value)
                            )
                        }
                    }
                }

                ThemedSpinBox {
                    theme: dpiPane.theme
                    enabled: dpiPane.controlsReady
                    from: dpiPane.controller.dpiMin
                    to: dpiPane.controller.dpiMax
                    stepSize: dpiPane.controller.dpiStep
                    editable: true
                    value: dpiPane.previewDpiY
                    Layout.preferredWidth: 128
                    onValueModified: {
                        dpiPane.previewDpiY = value
                        dpiPane.controller.setStageDpi(dpiPane.controller.activeStage, dpiPane.previewDpiX, value)
                    }
                }
            }

            // Stage LED Color Row
            RowLayout {
                Layout.fillWidth: true
                spacing: 12

                Text {
                    text: "Stage Color:"
                    color: dpiPane.theme.textMuted
                    font.pixelSize: 12
                    topPadding: 5
                    Layout.alignment: Qt.AlignTop | Qt.AlignLeft
                }

                ColorSwatchPicker {
                    Layout.fillWidth: true
                    theme: dpiPane.theme
                    controlsEnabled: dpiPane.controlsReady
                    selectedColor: dpiPane.controller.activeColor
                    onColorChosen: function(hex) {
                        dpiPane.controller.setStageColor(dpiPane.controller.activeStage, hex)
                    }
                }
            }
        }
    }

    // DPI Indicator LED Card
    Rectangle {
        Layout.fillWidth: true
        implicitHeight: ledCol.implicitHeight + 32
        radius: 12
        color: dpiPane.theme.bgCard
        border.color: dpiPane.theme.border
        border.width: 1

        ColumnLayout {
            id: ledCol
            anchors.fill: parent
            anchors.margins: 16
            spacing: 14

            RowLayout {
                Layout.fillWidth: true
                spacing: 10

                Text {
                    text: "DPI Indicator LED"
                    color: dpiPane.theme.textPrimary
                    font.pixelSize: 14
                    font.weight: Font.Bold
                }

                Item { Layout.fillWidth: true }

                Flow {
                    spacing: 6
                    Repeater {
                        model: [
                            { id: 0, label: "Off" },
                            { id: 1, label: "Static" },
                            { id: 2, label: "Breathing" }
                        ]
                        delegate: Button {
                            required property var modelData
                            readonly property bool selected: dpiPane.controller.dpiLedMode === modelData.id
                            enabled: dpiPane.controlsReady
                            hoverEnabled: true
                            implicitHeight: 28
                            text: modelData.label
                            onClicked: dpiPane.controller.setDpiLed(
                                modelData.id,
                                dpiPane.controller.dpiLedBrightness,
                                dpiPane.controller.dpiLedSpeed
                            )
                            contentItem: Text {
                                text: parent.text
                                color: parent.selected ? dpiPane.theme.textOnAccent
                                     : parent.enabled ? dpiPane.theme.textPrimary : dpiPane.theme.textDisabled
                                font.pixelSize: 12
                                font.weight: Font.DemiBold
                                leftPadding: 12
                                rightPadding: 12
                                verticalAlignment: Text.AlignVCenter
                                horizontalAlignment: Text.AlignHCenter
                            }
                            background: Rectangle {
                                radius: 6
                                color: parent.selected ? dpiPane.theme.accent
                                     : parent.hovered && parent.enabled ? dpiPane.theme.bgControlHover
                                     : dpiPane.theme.bgControl
                                border.color: parent.selected ? dpiPane.theme.accent : dpiPane.theme.borderControl
                            }
                        }
                    }
                }
            }

            GridLayout {
                Layout.fillWidth: true
                columns: dpiPane.width >= 640 ? 2 : 1
                columnSpacing: 20
                rowSpacing: 10

                RowLayout {
                    Layout.fillWidth: true
                    spacing: 10

                    Text {
                        text: "Brightness"
                        color: dpiPane.theme.textSecondary
                        font.pixelSize: 12
                        font.weight: Font.Medium
                        Layout.preferredWidth: 68
                    }

                    ThemedSlider {
                        id: brightnessSlider
                        theme: dpiPane.theme
                        Layout.fillWidth: true
                        enabled: dpiPane.controlsReady && dpiPane.controller.dpiLedMode !== 0
                        from: 0
                        to: 255
                        stepSize: 16
                        value: dpiPane.controller.dpiLedBrightness
                        onPressedChanged: {
                            if (!pressed && dpiPane.controlsReady) {
                                dpiPane.controller.setDpiLed(
                                    dpiPane.controller.dpiLedMode,
                                    Math.round(value),
                                    dpiPane.controller.dpiLedSpeed
                                )
                            }
                        }
                    }

                    Text {
                        text: Math.round(brightnessSlider.value / 255 * 100) + "%"
                        color: dpiPane.theme.textMuted
                        font.pixelSize: 11
                        font.family: "monospace"
                        Layout.preferredWidth: 36
                        horizontalAlignment: Text.AlignRight
                    }
                }

                RowLayout {
                    Layout.fillWidth: true
                    spacing: 10

                    Text {
                        text: "Speed"
                        color: dpiPane.theme.textSecondary
                        font.pixelSize: 12
                        font.weight: Font.Medium
                        Layout.preferredWidth: 68
                    }

                    ThemedSlider {
                        id: speedSlider
                        theme: dpiPane.theme
                        Layout.fillWidth: true
                        enabled: dpiPane.controlsReady && dpiPane.controller.dpiLedMode === 2
                        from: 1
                        to: 5
                        stepSize: 1
                        snapMode: Slider.SnapAlways
                        value: dpiPane.controller.dpiLedSpeed
                        onPressedChanged: {
                            if (!pressed && dpiPane.controlsReady) {
                                dpiPane.controller.setDpiLed(
                                    dpiPane.controller.dpiLedMode,
                                    dpiPane.controller.dpiLedBrightness,
                                    Math.round(value)
                                )
                            }
                        }
                    }

                    Text {
                        text: Math.round(speedSlider.value) + "/5"
                        color: dpiPane.theme.textMuted
                        font.pixelSize: 11
                        font.family: "monospace"
                        Layout.preferredWidth: 36
                        horizontalAlignment: Text.AlignRight
                    }
                }
            }
        }
    }
}
