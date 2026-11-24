{
  pkgs,
  lib,
  ...
}:

let
  qtEnv = pkgs.qt6.env "supernova-qt6-env" [
    pkgs.qt6.qtbase
    pkgs.qt6.qtdeclarative
    pkgs.qt6.qtwayland
    pkgs.qt6.qtsvg
    pkgs.qt6.qtshadertools
  ];
in
{
  languages.rust.enable = true;

  packages = [
    qtEnv
    pkgs.cmake
    pkgs.ninja
    pkgs.pkg-config
    pkgs.udev
    pkgs.libglvnd
    pkgs.gcc
    pkgs.mold
    pkgs.git
  ];

  env = {
    QMAKE = "${qtEnv}/bin/qmake";
    QT_PLUGIN_PATH = "${qtEnv}/lib/qt-6/plugins";
    QML2_IMPORT_PATH = "${qtEnv}/lib/qt-6/qml";
    QML_IMPORT_PATH = "${qtEnv}/lib/qt-6/qml";
    QT_QUICK_CONTROLS_STYLE = "Basic";
    RUST_BACKTRACE = "1";
  };

  enterShell = ''
    unset QT_QPA_PLATFORMTHEME QT_QPA_PLATFORMTHEME_QT6
    export LD_LIBRARY_PATH="${qtEnv}/lib:${lib.makeLibraryPath [ pkgs.udev pkgs.libglvnd ]}:''${LD_LIBRARY_PATH:-}"
  '';

  scripts = {
    build.exec = "cargo build --workspace";
    run.exec = ''cargo run -p supernova-qt --bin supernova -- "$@"'';
    cli.exec = ''cargo run -p supernova-core --bin supernova-cli -- "$@"'';
    test.exec = "cargo test --workspace";
    check.exec = "cargo check --workspace --all-targets";
    clippy.exec = "cargo clippy --workspace --all-targets -- -D warnings";
    fmt.exec = "cargo fmt --all -- --check";
  };
}
