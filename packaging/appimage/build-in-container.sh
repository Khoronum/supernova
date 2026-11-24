#!/usr/bin/env bash
set -euo pipefail

cd /src

export CARGO_HOME=/cache/cargo
export CARGO_TARGET_DIR=/cache/target
mkdir -p "$CARGO_HOME" "$CARGO_TARGET_DIR" /src/dist

cargo build --release --workspace

install -m 0755 /cache/target/release/supernova-cli /src/dist/supernova-cli
strip /src/dist/supernova-cli
install -m 0644 /src/udev/70-hator-mouse.rules /src/dist/70-hator-mouse.rules

rm -rf /tmp/AppDir
install -Dm 0755 /cache/target/release/supernova /tmp/AppDir/usr/bin/supernova
strip /tmp/AppDir/usr/bin/supernova
install -Dm 0755 /src/packaging/appimage/apprun-hook.sh /tmp/AppDir/apprun-hooks/99-supernova-env.sh

EXTRA_PLUGINS=""
for p in libqwayland-egl.so libqwayland-generic.so libqwayland.so libqxcb.so libqoffscreen.so; do
  if [ -f "$QT_DIR/plugins/platforms/$p" ]; then
    EXTRA_PLUGINS="${EXTRA_PLUGINS:+$EXTRA_PLUGINS;}$p"
  fi
done

export EXTRA_PLATFORM_PLUGINS="$EXTRA_PLUGINS"
export QML_SOURCES_PATHS=/src/crates/supernova-qt/qml
export LD_LIBRARY_PATH="$QT_DIR/lib:${LD_LIBRARY_PATH:-}"
export NO_STRIP=1

linuxdeploy \
  --appdir /tmp/AppDir \
  --desktop-file /src/packaging/appimage/supernova.desktop \
  --icon-file /src/packaging/appimage/supernova.svg \
  --plugin qt

EXTRA_LIBS=()
for subdir in wayland-shell-integration wayland-decoration-client wayland-graphics-integration-client; do
  if [ -d "$QT_DIR/plugins/$subdir" ] && [ ! -d "/tmp/AppDir/usr/plugins/$subdir" ]; then
    mkdir -p "/tmp/AppDir/usr/plugins/$subdir"
    cp -a "$QT_DIR/plugins/$subdir/"*.so "/tmp/AppDir/usr/plugins/$subdir/" 2>/dev/null || true
    for f in "/tmp/AppDir/usr/plugins/$subdir/"*.so; do
      [ -f "$f" ] && EXTRA_LIBS+=(--library "$f")
    done
  fi
done
if [ ${#EXTRA_LIBS[@]} -gt 0 ]; then
  linuxdeploy --appdir /tmp/AppDir "${EXTRA_LIBS[@]}"
fi

OUT_APPIMAGE="/src/dist/Supernova-x86_64.AppImage"
rm -f "$OUT_APPIMAGE" /src/Supernova*.AppImage /tmp/Supernova*.AppImage
(
  cd /tmp
  LDAI_OUTPUT="$OUT_APPIMAGE" linuxdeploy --appdir /tmp/AppDir --output appimage
)
if [ ! -f "$OUT_APPIMAGE" ]; then
  CANDIDATE="$(find /tmp /src -maxdepth 1 -name 'Supernova*.AppImage' | head -n 1 || true)"
  if [ -n "$CANDIDATE" ]; then
    mv "$CANDIDATE" "$OUT_APPIMAGE"
  else
    echo "error: AppImage output was not created" >&2
    exit 1
  fi
fi
chmod +x "$OUT_APPIMAGE"

echo "=== Verification Summary ==="
echo "CLI max GLIBC:"
objdump -T /src/dist/supernova-cli | grep -oE 'GLIBC_[0-9.]+' | sort -Vu | tail -n 5 || true
echo "GUI max GLIBC:"
objdump -T /tmp/AppDir/usr/bin/supernova | grep -oE 'GLIBC_[0-9.]+' | sort -Vu | tail -n 5 || true
echo "CLI ldd:"
ldd /src/dist/supernova-cli
echo "apprun-hooks:"
ls -la /tmp/AppDir/apprun-hooks/
echo "platforms:"
ls -la /tmp/AppDir/usr/plugins/platforms/
