# stickshift

Reads gear-shift events from a USB H-pattern shifter and runs customs shell script per gear change.
Works on Linux and macOS.

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
```
prints a table of connected HID devices with `VendorID`, `ProductID`, and
`Product` (name) columns. (`system_profiler SPUSBDataType` also works and
shows the same IDs in a more verbose per-device listing.)

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
first time `stickshift` runs, if they don't already exist. Each script is
run non-blocking (via `sh`), so a slow script won't block event
processing. See [`usage.md`](usage.md) for a cookbook of common script
snippets (opening URLs/files, controlling macOS apps via `osascript`, etc.).


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

## Logging

Controlled by `log_level` in `config.toml` (`error`/`warn`/`info`/`debug`/
`trace`), or override at runtime with the standard `env_logger` env var:

```sh
RUST_LOG=debug cargo run
```

To log to a file instead of the console:

```sh
cargo run 2> stickshift.log
```
