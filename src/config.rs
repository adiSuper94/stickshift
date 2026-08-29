use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

use serde::Deserialize;

const DEFAULT_CONFIG_TOML: &str = r#"device_name = "DragonRise Generic USB Joystick"
poll_interval_secs = 3
log_level = "info"
"#;

const GEARS: [&str; 7] = ["1", "2", "3", "4", "5", "6", "r"];

#[derive(Deserialize)]
#[serde(default)]
pub struct Config {
    pub device_name: String,
    pub poll_interval_secs: u64,
    pub log_level: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            device_name: "DragonRise Generic USB Joystick".to_string(),
            poll_interval_secs: 3,
            log_level: "info".to_string(),
        }
    }
}

pub fn stickshift_dir() -> PathBuf {
    let config_home = std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(std::env::var("HOME").expect("$HOME is not set")).join(".config")
        });
    config_home.join("stickshift")
}

pub fn ensure_layout(dir: &Path) {
    for (direction, preposition) in [("in", "into"), ("out", "out of")] {
        let sub_dir = dir.join("actions").join(direction);
        if let Err(e) = std::fs::create_dir_all(&sub_dir) {
            eprintln!("Failed to create {} ({e})", sub_dir.display());
            continue;
        }
        for gear in GEARS {
            let script_path = sub_dir.join(format!("{gear}.sh"));
            if script_path.exists() {
                continue;
            }
            let contents = format!(
                "#!/bin/sh\n# Runs when shifting {preposition} gear {}.\n",
                gear.to_uppercase()
            );
            if let Err(e) = std::fs::write(&script_path, &contents) {
                eprintln!("Failed to create {} ({e})", script_path.display());
                continue;
            }
            if let Err(e) =
                std::fs::set_permissions(&script_path, std::fs::Permissions::from_mode(0o755))
            {
                eprintln!("Failed to make {} executable ({e})", script_path.display());
            }
        }
    }

    let config_path = dir.join("config.toml");
    if !config_path.exists() {
        if let Err(e) = std::fs::write(&config_path, DEFAULT_CONFIG_TOML) {
            eprintln!("Failed to write default {} ({e})", config_path.display());
        }
    }
}

pub fn load_config(config_path: &Path) -> Config {
    match std::fs::read_to_string(config_path) {
        Ok(contents) => toml::from_str(&contents).unwrap_or_else(|e| {
            eprintln!(
                "Failed to parse {} ({e}), using defaults",
                config_path.display()
            );
            Config::default()
        }),
        Err(_) => Config::default(),
    }
}
