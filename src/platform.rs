use std::io;
use std::time::Duration;

use gilrs_core::{EvCode, EventType, Gilrs};
use log::info;

use crate::config::Config;

pub enum Gear {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Reverse,
}

impl Gear {
    pub fn label(self) -> &'static str {
        match self {
            Gear::First => "1",
            Gear::Second => "2",
            Gear::Third => "3",
            Gear::Fourth => "4",
            Gear::Fifth => "5",
            Gear::Sixth => "6",
            Gear::Reverse => "R",
        }
    }
}

#[cfg(target_os = "linux")]
fn to_gear(code: EvCode) -> (i32, Option<Gear>) {
    let button = (code.into_u32() & 0xFFFF) as i32;
    let gear = match button {
        288 => Some(Gear::First),
        289 => Some(Gear::Second),
        290 => Some(Gear::Third),
        291 => Some(Gear::Fourth),
        292 => Some(Gear::Fifth),
        293 => Some(Gear::Sixth),
        295 => Some(Gear::Reverse),
        _ => None,
    };
    (button, gear)
}

#[cfg(target_os = "macos")]
fn to_gear(code: EvCode) -> (i32, Option<Gear>) {
    let button = (code.into_u32() & 0xFFFF) as i32;
    let gear = match button {
        1 => Some(Gear::First),
        2 => Some(Gear::Second),
        3 => Some(Gear::Third),
        4 => Some(Gear::Fourth),
        5 => Some(Gear::Fifth),
        6 => Some(Gear::Sixth),
        8 => Some(Gear::Reverse),
        _ => None,
    };
    (button, gear)
}

pub enum GearEvent {
    Pressed(i32, Option<Gear>),
    Released(i32, Option<Gear>),
    Other,
}

pub struct Device {
    gilrs: Gilrs,
    id: usize,
}

fn matching_gamepad_id(gilrs: &Gilrs, vendor_id: u16, product_id: u16) -> Option<usize> {
    (0..gilrs.last_gamepad_hint()).find(|&id| {
        gilrs
            .gamepad(id)
            .is_some_and(|g| g.vendor_id() == Some(vendor_id) && g.product_id() == Some(product_id))
    })
}

pub fn find_device(config: &Config) -> io::Result<Device> {
    let gilrs = Gilrs::new().map_err(|e| io::Error::other(e.to_string()))?;
    match matching_gamepad_id(&gilrs, config.vendor_id, config.product_id) {
        Some(id) => {
            info!("Found device (gamepad id {id})");
            Ok(Device { gilrs, id })
        }
        None => Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!(
                "no gamepad with vendor={:04x} product={:04x} found; is it plugged in?",
                config.vendor_id, config.product_id
            ),
        )),
    }
}

pub fn wait_for_device(config: &Config) -> Device {
    let mut waiting_announced = false;
    loop {
        match find_device(config) {
            Ok(device) => return device,
            Err(_) => {
                if !waiting_announced {
                    info!(
                        "Waiting for vendor={:04x} product={:04x} to be connected...",
                        config.vendor_id, config.product_id
                    );
                    waiting_announced = true;
                }
                std::thread::sleep(Duration::from_secs(config.poll_interval_secs));
            }
        }
    }
}

pub fn next_events(device: &mut Device) -> io::Result<Vec<GearEvent>> {
    match device.gilrs.next_event_blocking(None) {
        Some(event) if event.id == device.id => Ok(vec![match event.event {
            EventType::ButtonPressed(code) => {
                let (button, gear) = to_gear(code);
                GearEvent::Pressed(button, gear)
            }
            EventType::ButtonReleased(code) => {
                let (button, gear) = to_gear(code);
                GearEvent::Released(button, gear)
            }
            EventType::Disconnected => {
                return Err(io::Error::new(io::ErrorKind::NotConnected, "device disconnected"));
            }
            _ => GearEvent::Other,
        }]),
        // An event from some other connected gamepad -- not ours, ignore it.
        Some(_) => Ok(vec![GearEvent::Other]),
        // Shouldn't happen with timeout=None, but don't treat it as an error.
        None => Ok(vec![]),
    }
}
