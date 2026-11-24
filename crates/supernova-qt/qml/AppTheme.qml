import QtQuick

QtObject {
    id: theme
    property bool dark: true

    readonly property color bgWindow: dark ? "#121216" : "#F3F4F7"
    readonly property color bgHeader: dark ? "#18181F" : "#FFFFFF"
    readonly property color bgSubHeader: dark ? "#16161C" : "#F8F9FB"
    readonly property color bgCard: dark ? "#1B1B22" : "#FFFFFF"
    readonly property color bgCardHover: dark ? "#252530" : "#F5F6FA"
    readonly property color bgCardDisabled: dark ? "#16161B" : "#ECEEF3"
    readonly property color bgControl: dark ? "#25252F" : "#ECEDF2"
    readonly property color bgControlHover: dark ? "#2F2F3D" : "#E0E2EA"
    readonly property color bgControlPressed: dark ? "#3A3A4A" : "#D3D6E0"
    readonly property color bgInput: dark ? "#141419" : "#F8F9FB"

    readonly property color border: dark ? "#2E2E3A" : "#DCDFE6"
    readonly property color borderSubtle: dark ? "#262632" : "#E6E8EE"
    readonly property color borderControl: dark ? "#3A3A4B" : "#C6CAD6"

    readonly property color accent: dark ? "#E1BD22" : "#D4A70A"
    readonly property color accentHover: dark ? "#ECCB3B" : "#C49907"
    readonly property color accentPressed: dark ? "#FFF1A8" : "#B08704"
    readonly property color accentSoft: dark ? "#25241D" : "#FDF6DB"
    readonly property color accentText: dark ? "#E1BD22" : "#8F6D00"
    readonly property color textOnAccent: "#121216"

    readonly property color textPrimary: dark ? "#F4F4F6" : "#16171D"
    readonly property color textSecondary: dark ? "#C4C4D0" : "#464957"
    readonly property color textMuted: dark ? "#8F8F9F" : "#6A6E7D"
    readonly property color textDisabled: dark ? "#6A6A7A" : "#9BA0AF"

    readonly property color trackOff: dark ? "#2C2C38" : "#D4D7E0"
    readonly property color knob: dark ? "#E8E8F0" : "#FFFFFF"

    readonly property color successBg: dark ? "#1D2620" : "#E8F7EE"
    readonly property color successBorder: dark ? "#2E4A36" : "#A5DAB8"
    readonly property color successText: dark ? "#6EE798" : "#176B3A"

    readonly property color warningBg: dark ? "#2A1F18" : "#FFF4E5"
    readonly property color warningBorder: dark ? "#4B3B2B" : "#F2C48D"
    readonly property color warningText: dark ? "#F6B97A" : "#9A4D00"

    readonly property color dangerBg: dark ? "#2C1B1E" : "#FDECEC"
    readonly property color dangerBgHover: dark ? "#3A2327" : "#FAD8D8"
    readonly property color dangerBorder: dark ? "#7A343B" : "#E09A9A"
    readonly property color dangerText: dark ? "#FF8A80" : "#B3261E"
}
