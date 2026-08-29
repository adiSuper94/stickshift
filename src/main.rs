use std::io;
use std::path::Path;
use std::process::{Command, Stdio};
use std::thread;

use log::{debug, info, warn};

mod config;
use config::{ensure_layout, load_config, stickshift_dir};

mod platform;
use platform::GearEvent;

fn run_action(actions_dir: &Path, direction: &str, gear: &str) {
    let script = actions_dir.join(direction).join(format!("{}.sh", gear.to_lowercase()));
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

fn handle_event(event: GearEvent, actions_dir: &Path) {
    match event {
        GearEvent::Pressed(button, gear) => match gear {
            Some(gear) => {
                let label = gear.label();
                info!("gear {label:<2} pressed  (button {button})");
                run_action(actions_dir, "in", label);
            }
            None => info!("button {button:<3} pressed"),
        },
        GearEvent::Released(button, gear) => match gear {
            Some(gear) => {
                let label = gear.label();
                info!("gear {label:<2} released (button {button})");
                run_action(actions_dir, "out", label);
            }
            None => info!("button {button:<3} released"),
        },
        GearEvent::Other => debug!("other event"),
    }
}

fn main() -> io::Result<()> {
    let stickshift_dir = stickshift_dir();
    ensure_layout(&stickshift_dir);
    let config = load_config(&stickshift_dir.join("config.toml"));
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(config.log_level.clone())).init();
    let actions_dir = stickshift_dir.join("actions");

    loop {
        let mut device = platform::wait_for_device(&config);
        info!("Logging events, press Ctrl+C to stop.");

        loop {
            let events = match platform::next_events(&mut device) {
                Ok(events) => events,
                Err(e) => {
                    warn!("Device disconnected ({e}), waiting for it to reconnect...");
                    break;
                }
            };
            for event in events {
                handle_event(event, &actions_dir);
            }
        }
    }
}
