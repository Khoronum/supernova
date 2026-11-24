import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import com.supernova.ui 1.0

ApplicationWindow {
    id: root
    visible: true
    width: 1040
    height: 720
    minimumWidth: 760
    minimumHeight: 560
    color: theme.bgWindow
    title: qsTr("Supernova")

    // 0 = Auto (system), 1 = Dark, 2 = Light
    property int themeMode: 0
    readonly property bool systemIsDark: Application.styleHints.colorScheme === Qt.ColorScheme.Dark
    readonly property bool effectiveDark: themeMode === 0 ? systemIsDark : (themeMode === 1)
    readonly property bool wideLayout: root.width >= 1160
    readonly property bool controlsReady: controller.connected && controller.mouseOnline

    AppTheme {
        id: theme
        dark: root.effectiveDark
    }

    palette.window: theme.bgWindow
    palette.windowText: theme.textPrimary
    palette.base: theme.bgInput
    palette.text: theme.textPrimary
    palette.button: theme.bgControl
    palette.buttonText: theme.textPrimary
    palette.highlight: theme.accent
    palette.highlightedText: theme.textOnAccent
    palette.mid: theme.border

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

    Dialog {
        id: resetDialog
        modal: true
        anchors.centerIn: Overlay.overlay
        width: Math.min(440, root.width - 40)
        padding: 18

        Overlay.modal: Rectangle {
            color: root.effectiveDark ? "#88000000" : "#55000000"
        }

        background: Rectangle {
            color: theme.bgCard
            radius: 12
            border.color: theme.borderControl
            border.width: 1
        }

        header: Item {
            implicitHeight: headerTitle.implicitHeight + 20
            Text {
                id: headerTitle
                anchors.left: parent.left
                anchors.right: parent.right
                anchors.top: parent.top
                anchors.leftMargin: 18
                anchors.rightMargin: 18
                anchors.topMargin: 16
                text: "Restore Factory Defaults"
                color: theme.textPrimary
                font.pixelSize: 16
                font.weight: Font.Bold
            }
            Rectangle {
                anchors.bottom: parent.bottom
                width: parent.width
                height: 1
                color: theme.borderSubtle
            }
        }

        contentItem: Label {
            text: "Reset all onboard mouse DPI stages, polling rate, and sensor settings to factory defaults?"
            color: theme.textSecondary
            font.pixelSize: 13
            wrapMode: Text.WordWrap
        }

        footer: Item {
            implicitHeight: dialogFooterRow.implicitHeight + 20
            Rectangle {
                anchors.top: parent.top
                width: parent.width
                height: 1
                color: theme.borderSubtle
            }
            RowLayout {
                id: dialogFooterRow
                anchors.right: parent.right
                anchors.verticalCenter: parent.verticalCenter
                anchors.rightMargin: 18
                spacing: 10

                Button {
                    text: "Cancel"
                    implicitHeight: 32
                    hoverEnabled: true
                    onClicked: resetDialog.reject()
                    contentItem: Text {
                        text: parent.text
                        color: theme.textPrimary
                        font.pixelSize: 12
                        font.weight: Font.Medium
                        leftPadding: 14
                        rightPadding: 14
                        horizontalAlignment: Text.AlignHCenter
                        verticalAlignment: Text.AlignVCenter
                    }
                    background: Rectangle {
                        radius: 6
                        color: parent.hovered ? theme.bgControlHover : theme.bgControl
                        border.color: theme.borderControl
                    }
                }

                Button {
                    text: "Reset Defaults"
                    implicitHeight: 32
                    hoverEnabled: true
                    onClicked: resetDialog.accept()
                    contentItem: Text {
                        text: parent.text
                        color: theme.dangerText
                        font.pixelSize: 12
                        font.weight: Font.Bold
                        leftPadding: 14
                        rightPadding: 14
                        horizontalAlignment: Text.AlignHCenter
                        verticalAlignment: Text.AlignVCenter
                    }
                    background: Rectangle {
                        radius: 6
                        color: parent.hovered ? theme.dangerBgHover : theme.dangerBg
                        border.color: theme.dangerBorder
                    }
                }
            }
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
            color: theme.bgHeader

            RowLayout {
                anchors.fill: parent
                anchors.leftMargin: 20
                anchors.rightMargin: 20
                spacing: 10

                // Brand Title
                RowLayout {
                    spacing: 10
                    Layout.minimumWidth: 160
                    Rectangle {
                        width: 10
                        height: 28
                        radius: 3
                        color: theme.accent
                    }
                    ColumnLayout {
                        spacing: 0
                        Text {
                            text: "Supernova"
                            color: theme.textPrimary
                            font.pixelSize: 16
                            font.weight: Font.Bold
                        }
                        Text {
                            text: controller.deviceName
                            color: theme.textMuted
                            font.pixelSize: 12
                            elide: Text.ElideRight
                            Layout.maximumWidth: 260
                        }
                    }
                }

                Item { Layout.fillWidth: true }

                // Status Badges
                Rectangle {
                    visible: controller.connected
                    color: theme.bgControl
                    radius: 6
                    border.color: theme.border
                    border.width: 1
                    implicitHeight: 28
                    implicitWidth: sensorBadgeText.implicitWidth + 18
                    Text {
                        id: sensorBadgeText
                        anchors.centerIn: parent
                        text: controller.sensorName + " · " + controller.connectionMode
                        color: theme.textSecondary
                        font.pixelSize: 11
                        font.weight: Font.Medium
                    }
                }

                Rectangle {
                    visible: controller.connected && controller.firmwareVersion !== "--"
                    color: theme.bgControl
                    radius: 6
                    border.color: theme.border
                    border.width: 1
                    implicitHeight: 28
                    implicitWidth: fwBadgeText.implicitWidth + 16
                    Text {
                        id: fwBadgeText
                        anchors.centerIn: parent
                        text: "FW " + controller.firmwareVersion
                        color: theme.textMuted
                        font.pixelSize: 11
                    }
                }

                Rectangle {
                    visible: controller.connected && controller.batteryPercent >= 0
                    color: theme.successBg
                    radius: 6
                    border.color: theme.successBorder
                    border.width: 1
                    implicitHeight: 28
                    implicitWidth: batText.implicitWidth + 18
                    Text {
                        id: batText
                        anchors.centerIn: parent
                        text: (controller.batteryCharging ? "⚡ " : "") + controller.batteryPercent + "%"
                        color: controller.batteryPercent <= 15 ? theme.dangerText : theme.successText
                        font.pixelSize: 11
                        font.weight: Font.Bold
                    }
                }

                Button {
                    text: root.themeMode === 0
                          ? ("Theme: Auto (" + (root.effectiveDark ? "Dark" : "Light") + ")")
                          : (root.themeMode === 1 ? "Theme: Dark" : "Theme: Light")
                    implicitHeight: 32
                    hoverEnabled: true
                    onClicked: root.themeMode = (root.themeMode + 1) % 3
                    contentItem: Text {
                        text: parent.text
                        color: theme.textSecondary
                        font.pixelSize: 11
                        font.weight: Font.Medium
                        horizontalAlignment: Text.AlignHCenter
                        verticalAlignment: Text.AlignVCenter
                        leftPadding: 10
                        rightPadding: 10
                    }
                    background: Rectangle {
                        radius: 6
                        color: parent.hovered ? theme.bgControlHover : theme.bgControl
                        border.color: theme.borderControl
                    }
                }

                Button {
                    text: "Refresh"
                    implicitHeight: 32
                    hoverEnabled: true
                    onClicked: controller.refreshDevice()
                    contentItem: Text {
                        text: parent.text
                        color: theme.textPrimary
                        font.pixelSize: 12
                        font.weight: Font.Medium
                        horizontalAlignment: Text.AlignHCenter
                        verticalAlignment: Text.AlignVCenter
                        leftPadding: 10
                        rightPadding: 10
                    }
                    background: Rectangle {
                        radius: 6
                        color: parent.hovered ? theme.bgControlHover : theme.bgControl
                        border.color: theme.borderControl
                    }
                }

                Button {
                    text: controller.demoMode ? "Exit Demo" : "Demo Mode"
                    implicitHeight: 32
                    hoverEnabled: true
                    onClicked: controller.applyDemoMode(!controller.demoMode)
                    contentItem: Text {
                        text: parent.text
                        color: controller.demoMode ? theme.textOnAccent : theme.accentText
                        font.pixelSize: 12
                        font.weight: Font.DemiBold
                        horizontalAlignment: Text.AlignHCenter
                        verticalAlignment: Text.AlignVCenter
                        leftPadding: 10
                        rightPadding: 10
                    }
                    background: Rectangle {
                        radius: 6
                        color: controller.demoMode ? theme.accent
                             : (parent.hovered ? theme.accentSoft : theme.bgControl)
                        border.color: theme.accent
                    }
                }
            }

            Rectangle {
                anchors.bottom: parent.bottom
                width: parent.width
                height: 1
                color: theme.border
            }
        }

        // Permission / Offline Banner
        Rectangle {
            Layout.fillWidth: true
            visible: !root.controlsReady
            color: controller.permissionDenied ? theme.warningBg : theme.bgSubHeader
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
                        color: controller.permissionDenied ? theme.warningText : theme.textPrimary
                        font.pixelSize: 13
                        font.weight: Font.Medium
                        wrapMode: Text.WordWrap
                    }

                    Button {
                        visible: !controller.demoMode
                        text: "Try Demo Mode"
                        implicitHeight: 30
                        hoverEnabled: true
                        onClicked: controller.applyDemoMode(true)
                        contentItem: Text {
                            text: parent.text
                            color: theme.textOnAccent
                            font.pixelSize: 12
                            font.weight: Font.Bold
                            horizontalAlignment: Text.AlignHCenter
                            verticalAlignment: Text.AlignVCenter
                            leftPadding: 10
                            rightPadding: 10
                        }
                        background: Rectangle {
                            radius: 6
                            color: parent.hovered ? theme.accentHover : theme.accent
                        }
                    }
                }

                TextField {
                    visible: controller.permissionDenied
                    Layout.fillWidth: true
                    readOnly: true
                    selectByMouse: true
                    text: controller.udevHelpCommand
                    color: theme.textPrimary
                    selectionColor: theme.accent
                    selectedTextColor: theme.textOnAccent
                    font.family: "monospace"
                    font.pixelSize: 11
                    background: Rectangle {
                        color: theme.bgInput
                        radius: 6
                        border.color: controller.permissionDenied ? theme.warningBorder : theme.borderControl
                    }
                }
            }

            Rectangle {
                anchors.bottom: parent.bottom
                width: parent.width
                height: 1
                color: controller.permissionDenied ? theme.warningBorder : theme.border
            }
        }

        // Navigation Tabs (compact windows only)
        TabBar {
            id: tabBar
            visible: !root.wideLayout
            Layout.fillWidth: true
            background: Rectangle {
                color: theme.bgSubHeader
                Rectangle {
                    anchors.bottom: parent.bottom
                    width: parent.width
                    height: 1
                    color: theme.border
                }
            }

            TabButton {
                text: "DPI Settings"
                width: 170
                height: 42
                contentItem: Text {
                    text: parent.text
                    color: tabBar.currentIndex === 0 ? theme.accentText : theme.textMuted
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
                        color: theme.accent
                    }
                }
            }

            TabButton {
                text: "Performance & Sensor"
                width: 190
                height: 42
                contentItem: Text {
                    text: parent.text
                    color: tabBar.currentIndex === 1 ? theme.accentText : theme.textMuted
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
                        color: theme.accent
                    }
                }
            }
        }

        // Responsive Main Content (Tabbed when compact, Side-by-Side when >= 1160px)
        ScrollView {
            id: mainScroll
            Layout.fillWidth: true
            Layout.fillHeight: true
            clip: true
            contentWidth: availableWidth
            ScrollBar.horizontal.policy: ScrollBar.AlwaysOff
            ScrollBar.vertical: ScrollBar {
                policy: ScrollBar.AsNeeded
                contentItem: Rectangle {
                    implicitWidth: 6
                    radius: 3
                    color: theme.borderControl
                }
            }

            Item {
                id: scrollContent
                width: mainScroll.availableWidth
                implicitHeight: panesFrame.implicitHeight + 32

                GridLayout {
                    id: panesFrame
                    x: Math.max(20, (scrollContent.width - width) / 2)
                    y: 16
                    width: Math.min(Math.max(1, scrollContent.width - 40), 1520)
                    columns: root.wideLayout ? 2 : 1
                    columnSpacing: 20
                    rowSpacing: 16

                    DpiSettingsPane {
                        id: dpiPane
                        visible: root.wideLayout || tabBar.currentIndex === 0
                        showSectionHeader: root.wideLayout
                        controller: controller
                        theme: theme
                        controlsReady: root.controlsReady
                        Layout.fillWidth: true
                        Layout.preferredWidth: root.wideLayout ? 560 : 1
                        Layout.alignment: Qt.AlignTop
                    }

                    PerformancePane {
                        id: perfPane
                        visible: root.wideLayout || tabBar.currentIndex === 1
                        showSectionHeader: root.wideLayout
                        controller: controller
                        theme: theme
                        controlsReady: root.controlsReady
                        Layout.fillWidth: true
                        Layout.preferredWidth: root.wideLayout ? 440 : 1
                        Layout.alignment: Qt.AlignTop
                        onRequestFactoryReset: resetDialog.open()
                    }
                }
            }
        }

        // Footer Status Bar
        Rectangle {
            Layout.fillWidth: true
            Layout.preferredHeight: 30
            color: theme.bgSubHeader

            Rectangle {
                anchors.top: parent.top
                width: parent.width
                height: 1
                color: theme.border
            }

            RowLayout {
                anchors.fill: parent
                anchors.leftMargin: 16
                anchors.rightMargin: 16
                spacing: 12

                Text {
                    Layout.fillWidth: true
                    text: controller.errorMessage.length > 0 ? controller.errorMessage : controller.statusMessage
                    color: controller.errorMessage.length > 0 ? theme.warningText : theme.textMuted
                    font.pixelSize: 11
                    elide: Text.ElideRight
                }
                Text {
                    text: "VID 0x379A · Report 0x08"
                    color: theme.textMuted
                    font.pixelSize: 11
                }
            }
        }
    }
}
