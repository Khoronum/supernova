import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import com.supernova.ui 1.0

ColumnLayout {
    id: perfPane
    required property var controller
    required property AppTheme theme
    required property bool controlsReady
    property bool showSectionHeader: false
    readonly property var supportedPollingRates: {
        try {
            let parsed = JSON.parse(controller.supportedPollingRatesJson)
            return Array.isArray(parsed) && parsed.length > 0 ? parsed : [125, 250, 500, 1000]
        } catch (e) {
            return [125, 250, 500, 1000]
        }
    }

    signal requestFactoryReset()

    spacing: 14

    Text {
        visible: perfPane.showSectionHeader
        text: "PERFORMANCE & SENSOR"
        color: perfPane.theme.textMuted
        font.pixelSize: 11
        font.weight: Font.Bold
        font.letterSpacing: 1.0
    }

    // Polling Rate Card
    Rectangle {
        Layout.fillWidth: true
        implicitHeight: rateCol.implicitHeight + 32
        radius: 12
        color: perfPane.theme.bgCard
        border.color: perfPane.theme.border
        border.width: 1

        ColumnLayout {
            id: rateCol
            anchors.fill: parent
            anchors.margins: 16
            spacing: 12

            RowLayout {
                Layout.fillWidth: true
                spacing: 8

                Text {
                    text: "USB Polling Rate"
                    color: perfPane.theme.textPrimary
                    font.pixelSize: 15
                    font.weight: Font.Bold
                }

                Item { Layout.fillWidth: true }

                Text {
                    visible: perfPane.controller.connected && perfPane.controller.pollingRateLimitNote.length > 0
                    text: perfPane.controller.pollingRateLimitNote
                    color: perfPane.theme.textMuted
                    font.pixelSize: 11
                    font.weight: Font.Medium
                }
            }

            GridLayout {
                Layout.fillWidth: true
                columns: perfPane.width >= 580
                         ? Math.max(1, perfPane.supportedPollingRates.length)
                         : Math.min(4, Math.max(1, perfPane.supportedPollingRates.length))
                columnSpacing: 8
                rowSpacing: 8

                Repeater {
                    model: perfPane.supportedPollingRates
                    delegate: Button {
                        required property int modelData
                        readonly property bool selected: perfPane.controller.pollingRateHz === modelData
                        Layout.fillWidth: true
                        implicitHeight: 36
                        hoverEnabled: true
                        enabled: perfPane.controlsReady
                        text: modelData >= 1000 ? ((modelData / 1000) + "000 Hz") : (modelData + " Hz")
                        onClicked: perfPane.controller.setPollingRate(modelData)
                        contentItem: Text {
                            text: parent.text
                            color: parent.selected ? perfPane.theme.textOnAccent
                                 : parent.enabled ? perfPane.theme.textPrimary : perfPane.theme.textDisabled
                            font.pixelSize: 12
                            font.weight: Font.Bold
                            horizontalAlignment: Text.AlignHCenter
                            verticalAlignment: Text.AlignVCenter
                        }
                        background: Rectangle {
                            radius: 8
                            color: parent.selected ? perfPane.theme.accent
                                 : parent.hovered && parent.enabled ? perfPane.theme.bgControlHover
                                 : perfPane.theme.bgControl
                            border.color: parent.selected ? perfPane.theme.accent : perfPane.theme.borderControl
                        }
                    }
                }
            }

            Text {
                Layout.fillWidth: true
                visible: perfPane.controlsReady && perfPane.controller.pollingRateHz > perfPane.controller.maxPollingRateHz
                text: "\u26A0 Current onboard rate (" + perfPane.controller.pollingRateHz + " Hz) exceeds this connection's "
                      + perfPane.controller.maxPollingRateHz + " Hz limit. Select a supported rate above."
                color: perfPane.theme.warningText
                font.pixelSize: 12
                wrapMode: Text.WordWrap
            }
        }
    }

    // Sensor Tuning & LOD Card
    Rectangle {
        Layout.fillWidth: true
        visible: perfPane.controller.supportsLod
              || perfPane.controller.supportsMotionSync
              || perfPane.controller.supportsAngleSnapping
              || perfPane.controller.supportsRippleControl
              || perfPane.controller.supportsHighPerformance
        implicitHeight: sensorCol.implicitHeight + 32
        radius: 12
        color: perfPane.theme.bgCard
        border.color: perfPane.theme.border
        border.width: 1

        ColumnLayout {
            id: sensorCol
            anchors.fill: parent
            anchors.margins: 16
            spacing: 14

            Text {
                text: perfPane.controller.supportsLod
                      ? "Sensor Features & Lift-Off Distance"
                      : "Sensor Features"
                color: perfPane.theme.textPrimary
                font.pixelSize: 15
                font.weight: Font.Bold
            }

            RowLayout {
                Layout.fillWidth: true
                visible: perfPane.controller.supportsLod
                spacing: 10

                Text {
                    text: "Lift-Off Distance (LOD):"
                    color: perfPane.theme.textSecondary
                    font.pixelSize: 13
                    topPadding: 4
                    Layout.alignment: Qt.AlignTop | Qt.AlignLeft
                }

                Flow {
                    Layout.fillWidth: true
                    spacing: 8

                    Repeater {
                        model: [
                            { code: 3, label: "0.7 mm", needSubMm: true },
                            { code: 1, label: "1.0 mm", needSubMm: false },
                            { code: 2, label: "2.0 mm", needSubMm: false }
                        ]
                        delegate: Button {
                            required property var modelData
                            readonly property bool selected: perfPane.controller.lodLevel === modelData.code
                            visible: !modelData.needSubMm || perfPane.controller.supportsSubMmLod
                            enabled: perfPane.controlsReady
                            hoverEnabled: true
                            implicitHeight: 30
                            text: modelData.label
                            onClicked: perfPane.controller.setLod(modelData.code)
                            contentItem: Text {
                                text: parent.text
                                color: parent.selected ? perfPane.theme.textOnAccent
                                     : parent.enabled ? perfPane.theme.textPrimary : perfPane.theme.textDisabled
                                font.pixelSize: 12
                                font.weight: Font.Bold
                                leftPadding: 14
                                rightPadding: 14
                                horizontalAlignment: Text.AlignHCenter
                                verticalAlignment: Text.AlignVCenter
                            }
                            background: Rectangle {
                                radius: 6
                                color: parent.selected ? perfPane.theme.accent
                                     : parent.hovered && parent.enabled ? perfPane.theme.bgControlHover
                                     : perfPane.theme.bgControl
                                border.color: parent.selected ? perfPane.theme.accent : perfPane.theme.borderControl
                            }
                        }
                    }
                }
            }

            GridLayout {
                Layout.fillWidth: true
                columns: perfPane.width >= 430 ? 2 : 1
                columnSpacing: 14
                rowSpacing: 6

                ThemedSwitch {
                    theme: perfPane.theme
                    Layout.fillWidth: true
                    visible: perfPane.controller.supportsMotionSync
                    enabled: perfPane.controlsReady
                    checked: perfPane.controller.motionSync
                    text: "Motion Sync"
                    onToggled: perfPane.controller.applyMotionSync(checked)
                }
                ThemedSwitch {
                    theme: perfPane.theme
                    Layout.fillWidth: true
                    visible: perfPane.controller.supportsAngleSnapping
                    enabled: perfPane.controlsReady
                    checked: perfPane.controller.angleSnapping
                    text: "Angle Snapping"
                    onToggled: perfPane.controller.applyAngleSnapping(checked)
                }
                ThemedSwitch {
                    theme: perfPane.theme
                    Layout.fillWidth: true
                    visible: perfPane.controller.supportsRippleControl
                    enabled: perfPane.controlsReady
                    checked: perfPane.controller.rippleControl
                    text: "Ripple Control"
                    onToggled: perfPane.controller.applyRippleControl(checked)
                }
                ThemedSwitch {
                    theme: perfPane.theme
                    Layout.fillWidth: true
                    visible: perfPane.controller.supportsHighPerformance
                    enabled: perfPane.controlsReady
                    checked: perfPane.controller.highPerformance
                    text: "High Performance Mode"
                    onToggled: perfPane.controller.applyHighPerformance(checked)
                }
            }
        }
    }

    // Debounce, Sleep, and Reset Card
    Rectangle {
        Layout.fillWidth: true
        implicitHeight: timingCol.implicitHeight + 32
        radius: 12
        color: perfPane.theme.bgCard
        border.color: perfPane.theme.border
        border.width: 1

        ColumnLayout {
            id: timingCol
            anchors.fill: parent
            anchors.margins: 16
            spacing: 12

            Text {
                text: "Button Debounce, Sleep & Factory Reset"
                color: perfPane.theme.textPrimary
                font.pixelSize: 15
                font.weight: Font.Bold
            }

            RowLayout {
                Layout.fillWidth: true
                spacing: 12

                Text {
                    text: "Debounce Time:"
                    color: perfPane.theme.textSecondary
                    font.pixelSize: 13
                    Layout.preferredWidth: 112
                }

                ThemedSpinBox {
                    theme: perfPane.theme
                    enabled: perfPane.controlsReady
                    from: 0
                    to: 30
                    value: perfPane.controller.debounceMs
                    onValueModified: perfPane.controller.applyDebounceMs(value)
                }

                Text {
                    Layout.fillWidth: true
                    text: perfPane.controller.debounceMs < 3
                          ? "ms (\u26A0 < 3 ms may cause double-clicking)"
                          : "ms"
                    color: perfPane.controller.debounceMs < 3
                           ? perfPane.theme.warningText
                           : perfPane.theme.textMuted
                    font.pixelSize: 12
                    wrapMode: Text.WordWrap
                }
            }

            RowLayout {
                Layout.fillWidth: true
                spacing: 12

                Text {
                    text: "Sleep Timeout:"
                    color: perfPane.theme.textSecondary
                    font.pixelSize: 13
                    Layout.preferredWidth: 112
                }

                ThemedSpinBox {
                    theme: perfPane.theme
                    enabled: perfPane.controlsReady
                    from: 1
                    to: 40
                    value: perfPane.controller.sleepMinutes
                    onValueModified: perfPane.controller.applySleepMinutes(value)
                }

                Text {
                    Layout.fillWidth: true
                    text: "minutes"
                    color: perfPane.theme.textMuted
                    font.pixelSize: 12
                }
            }

            Rectangle {
                Layout.fillWidth: true
                Layout.topMargin: 2
                Layout.bottomMargin: 2
                height: 1
                color: perfPane.theme.borderSubtle
            }

            RowLayout {
                Layout.fillWidth: true
                spacing: 12

                Text {
                    Layout.fillWidth: true
                    text: "Restore onboard mouse flash settings to factory defaults."
                    color: perfPane.theme.textMuted
                    font.pixelSize: 12
                    wrapMode: Text.WordWrap
                }

                Button {
                    enabled: perfPane.controlsReady
                    hoverEnabled: true
                    text: "Factory Reset"
                    implicitHeight: 34
                    opacity: enabled ? 1.0 : 0.48
                    onClicked: perfPane.requestFactoryReset()
                    contentItem: Text {
                        text: parent.text
                        color: perfPane.theme.dangerText
                        font.pixelSize: 12
                        font.weight: Font.Bold
                        leftPadding: 14
                        rightPadding: 14
                        horizontalAlignment: Text.AlignHCenter
                        verticalAlignment: Text.AlignVCenter
                    }
                    background: Rectangle {
                        radius: 6
                        color: parent.hovered && parent.enabled
                               ? perfPane.theme.dangerBgHover
                               : perfPane.theme.dangerBg
                        border.color: perfPane.theme.dangerBorder
                    }
                }
            }
        }
    }
}
