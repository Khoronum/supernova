pub mod controller;

use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QString, QUrl};

fn main() {
    let mut app = QGuiApplication::new();
    if let Some(mut app_pin) = app.as_mut() {
        app_pin
            .as_mut()
            .set_application_name(&QString::from("supernova"));
        app_pin
            .as_mut()
            .set_application_display_name(&QString::from("Supernova"));
        app_pin
            .as_mut()
            .set_organization_name(&QString::from("Supernova"));
    }

    let mut engine = QQmlApplicationEngine::new();
    if let Some(engine_pin) = engine.as_mut() {
        engine_pin.load(&QUrl::from("qrc:/qt/qml/com/supernova/ui/qml/Main.qml"));
    }

    if let Some(app_pin) = app.as_mut() {
        let code = app_pin.exec();
        std::process::exit(code);
    }
}
