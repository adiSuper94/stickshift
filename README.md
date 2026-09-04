# stickshift

Reads gear-shift events from a USB H-pattern shifter and runs customs shell script per gear change.
Works on Linux and macOS.

### Installation

Downloads the latest release for your OS/arch to `~/.local/bin/stickshift` and installs +
enables the background service (systemd user unit on Linux, launchd agent on macOS):

```sh
curl -fsSL https://raw.githubusercontent.com/adiSuper94/stickshift/main/install.sh | sh
```

Re-running it upgrades the binary in place and restarts the service.

## Configuration

On first run, `stickshift` creates `~/.config/stickshift/` with a default `config.toml`
and a stub `actions/` directory.

`~/.config/stickshift/config.toml`:

```toml
device_name = "DragonRise Generic USB Joystick"  # label only, used for logging
vendor_id = 0x0079
product_id = 0x0006
poll_interval_secs = 3
log_level = "info"
```

The device is matched by `vendor_id`/`product_id`, not by name.
`device_name` is only ever printed in logs.

### Finding your device's vendor ID, product ID, and name

**Linux:**

```sh
lsusb
# ...
# Bus 001 Device 005: ID 0079:0006 DragonRise Inc. Generic USB Joystick
# ...
```

The `ID` field is `vendor_id:product_id` in hex, followed by the name.

**macOS:**

```sh
hidutil list
# Services:
#...
#
# Devices:
# VendorID ProductID LocationID UsagePage Usage RegistryID  Transport Class                 Product                 UserClass                 Built-In
# ...
# 0x79     0x6       0x1141000  1         4     0x100046f31 USB       AppleUserHIDDevice    Generic USB Joystick    AppleUserUSBHostHIDDevice 0
# ...
```
prints a table of connected HID devices with `VendorID`, `ProductID`, and
`Product` (name) columns.

### Gear-triggered scripts

Shifting into a gear runs the matching script under `actions/in/`;
shifting out of it runs the one under `actions/out/`:

```
.config/stickshift
├── actions
│   ├── in
│   │   ├── 1.sh
│   │   ├── ...
│   │   ├── 6.sh
│   │   └── r.sh
│   └── out
│       ├── 1.sh
│       ├── ...
│       ├── 6.sh
│       └── r.sh
└── config.toml
```

Stub scripts for all 6 gears + reverse are created automatically the
first time `stickshift` runs, if they don't already exist. See [`usage.md`](usage.md) for a cookbook of common script
snippets.


## Building

With [Nix](https://nixos.org) (recommended):

```sh
nix build .#default    # -> result/bin/stickshift
```

You can also build with `cargo build --release`. But you need to ensure that you have all the
dependencies.

### CI-built binaries

GH actions builds `aarch64-linux`, `x86_64-linux`, and `aarch64-darwin` (Apple Silicon) via
`nix build .#default` on GitHub-hosted runners.

## Running

```sh
cargo run
```

## Uninstalling 😢

```sh
stickshift uninstall
```

Stops and removes the background service, and deletes the `stickshift` binary itself. Your
config scripts in `~/.config/stickshift/` are left in place; remove those manually if
you want them gone too.

