use std::io;
use std::path::Path;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

use evdev::{Device, EventSummary, InputEvent};
use log::{debug, info, warn};

mod config;
use config::{ensure_layout, load_config, stickshift_dir};

fn run_action(actions_dir: &Path, direction: &str, gear: &str) {
    let script = actions_dir
        .join(direction)
        .join(format!("{}.sh", gear.to_lowercase()));
    if !script.is_file() {
        return;
    }
    match Command::new("sh").arg(&script).stdin(Stdio::null()).spawn() {
        Ok(mut child) => {
            thread::spawn(move || match child.wait() {
                Ok(status) if status.success() => {
                    debug!("{} finished successfully", script.display())
                }
                Ok(status) => warn!("{} exited with {status}", script.display()),
                Err(e) => warn!("failed to wait on {}: {e}", script.display()),
            });
        }
        Err(e) => warn!("failed to run {}: {e}", script.display()),
    }
}

fn gear_name(hid_button: i32) -> Option<&'static str> {
    match hid_button {
        1 => Some("1"),
        2 => Some("2"),
        3 => Some("3"),
        4 => Some("4"),
        5 => Some("5"),
        6 => Some("6"),
        8 => Some("R"),
        _ => None,
    }
}

fn find_device(device_name: &str) -> io::Result<Device> {
    for (path, device) in evdev::enumerate() {
        if device.name().map(str::trim) == Some(device_name) {
            info!("Found \"{device_name}\" at {}", path.display());
            return Ok(device);
        }
    }
    Err(io::Error::new(
        io::ErrorKind::NotFound,
        format!("no input device named \"{device_name}\" found; is it plugged in?"),
    ))
}

fn wait_for_device(device_name: &str, poll_interval: Duration) -> Device {
    let mut waiting_announced = false;
    loop {
        match find_device(device_name) {
            Ok(device) => return device,
            Err(_) => {
                if !waiting_announced {
                    info!("Waiting for \"{device_name}\" to be connected...");
                    waiting_announced = true;
                }
                thread::sleep(poll_interval);
            }
        }
    }
}

fn handle_event(event: InputEvent, pending_scan: &mut Option<i32>, actions_dir: &Path) {
    match event.destructure() {
        EventSummary::Misc(_, _, value) => *pending_scan = Some(value),
        EventSummary::Key(_, key, value) => {
            let action = match value {
                0 => "released",
                1 => "pressed",
                2 => "repeated",
                _ => "?",
            };
            match pending_scan.take() {
                Some(scan) => {
                    let hid_button = scan & 0xFF;
                    match gear_name(hid_button) {
                        Some(gear) => {
                            info!("gear {gear:<2} {action:<8} (button {hid_button}, {key:?})");
                            match action {
                                "pressed" => run_action(actions_dir, "in", gear),
                                "released" => run_action(actions_dir, "out", gear),
                                _ => {}
                            }
                        }
                        None => info!("button {hid_button:<3} ({key:?}) {action}"),
                    }
                }
                None => info!("{key:?} {action}"),
            }
        }
        EventSummary::Synchronization(..) => {}
        other => debug!("{other:?}"),
    }
}

fn main() -> io::Result<()> {
    let stickshift_dir = stickshift_dir();
    ensure_layout(&stickshift_dir);
    let config = load_config(&stickshift_dir.join("config.toml"));
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or(config.log_level.clone()),
    )
    .init();
    let actions_dir = stickshift_dir.join("actions");
    let poll_interval = Duration::from_secs(config.poll_interval_secs);

    loop {
        let mut device = wait_for_device(&config.device_name, poll_interval);
        info!("Logging events, press Ctrl+C to stop.");

        // The device emits an EV_MSC/MSC_SCAN event carrying the raw HID button
        // number immediately before the EV_KEY event for the same physical button,
        // so `pending_scan` stashes it and we print both together once the Key
        // event arrives.
        let mut pending_scan: Option<i32> = None;

        loop {
            let events = match device.fetch_events() {
                Ok(events) => events,
                Err(e) => {
                    warn!("Device disconnected ({e}), waiting for it to reconnect...");
                    break;
                }
            };
            for event in events {
                handle_event(event, &mut pending_scan, &actions_dir);
            }
        }
    }
}
