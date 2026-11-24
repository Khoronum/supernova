use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    CxxQtBuilder::new_qml_module(QmlModule::new("com.supernova.ui").qml_files([
        "qml/Main.qml",
        "qml/AppTheme.qml",
        "qml/DpiSettingsPane.qml",
        "qml/PerformancePane.qml",
        "qml/DpiStageCard.qml",
        "qml/ColorSwatchPicker.qml",
        "qml/ThemedSwitch.qml",
        "qml/ThemedSpinBox.qml",
        "qml/ThemedSlider.qml",
    ]))
    .qt_module("QuickControls2")
    .files(["src/controller.rs"])
    .build();
}
