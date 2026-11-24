# Supernova - a configuration utility for Linux

> NOTICE: Made with an LLM. 

Native Linux configuration utility for HATOR gaming mice. Includes a CLI based `supernova-cli` and QT6 interface.

## Features
- **DPI Stages & Active Stage Switching**: Configure up to 8 DPI stages (6 enabled by default), switch active stage instantaneously, adjust independent X/Y DPI (on supported sensors), and set per-stage indicator LED colors.
- **Sensor Support**: Supports features in the following sensors:
  - `PAW3950` (`50–30,000` DPI in 50-DPI steps, independent X/Y, 0.7 / 1 / 2 mm LOD)
  - `PAW3395` (`50–26,000` DPI in 50-DPI steps, independent X/Y, 0.7 / 1 / 2 mm LOD)
  - `PAW3311` (`50–12,000` / `50–24,000` DPI using exact sensor register LUT & step tiers)
  - `PAW3335`, `PAW3325`, `PAW3370`
- **Performance & Sensor Tuning**: Polling rate (`125`–`8000` Hz), Lift-Off Distance (`0.7` / `1.0` / `2.0` mm), Motion Sync, Angle Snapping, Ripple Control, High Performance Mode, Debounce Time (`0–30` ms), Sleep Timer, DPI Indicator LED mode/brightness/speed, and Factory Reset.
- **Supported VID/PIDs** (`VID 0x379A`):
  - Wired: `0x0CEF`, `0x0CEB`, `0x3607`, `0x3318`, `0x3319`, `0x3307`, `0x3321`, `0x3330`, `0x3327`
  - 2.4 GHz Wireless: `0x0CF0`,  `0x3608`, `0x3308`, `0x3322`, `0x3331`, `0x3328`
  - 8K Wireless: `0x0CEC`

## USB HID Permissions (`udev`)

Linux restricts `/dev/hidraw*` to `root` by default. Either run commands via root or install the provided udev rule and reload:

```bash
sudo cp udev/70-hator-mouse.rules /etc/udev/rules.d/
sudo udevadm control --reload-rules
sudo udevadm trigger --subsystem-match=hidraw --subsystem-match=usb
```

> **Note:** If you previously installed `99-hator-mouse.rules`, remove `/etc/udev/rules.d/99-hator-mouse.rules` because `TAG+="uaccess"` must be evaluated before systemd's `73-seat-late.rules`.

Unplug and replug the mouse or wireless receiver if needed.

## Building & Running (via `devenv`)

```bash
# Enter the devenv shell or prefix commands with `devenv shell --`
devenv shell -- cargo test --workspace
devenv shell -- cargo build --workspace

# Launch the Qt 6 GUI
devenv shell -- cargo run -p supernova-qt --bin supernova

# Launch GUI in offline Demo Mode (no hardware/udev permissions needed)
devenv shell -- cargo run -p supernova-qt --bin supernova -- --demo
```

### CLI Usage (`supernova-cli`)
> todo: this is dumb
```bash
# List detected HATOR HID interfaces
devenv shell -- cargo run -p supernova-core --bin supernova-cli -- list

# Inspect device status, battery, firmware, polling rate, and DPI stages
devenv shell -- cargo run -p supernova-core --bin supernova-cli -- status

# Switch active DPI stage (1-indexed)
devenv shell -- cargo run -p supernova-core --bin supernova-cli -- dpi stage 3

# Set Stage 3 to 1600 DPI with cyan indicator color
devenv shell -- cargo run -p supernova-core --bin supernova-cli -- dpi set 3 1600 --color '#00FFFF'

# Set polling rate to 1000 Hz
devenv shell -- cargo run -p supernova-core --bin supernova-cli -- rate 1000
```


