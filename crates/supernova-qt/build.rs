use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    CxxQtBuilder::new_qml_module(QmlModule::new("com.supernova.ui").qml_files([
        "qml/Main.qml",
        "qml/DpiStageCard.qml",
        "qml/ColorSwatchPicker.qml",
    ]))
    .qt_module("QuickControls2")
    .files(["src/controller.rs"])
    .build();
}
