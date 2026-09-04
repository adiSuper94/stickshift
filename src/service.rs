use std::io;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use log::{info, warn};

fn home_dir() -> io::Result<PathBuf> {
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "$HOME is not set"))
}

fn remove_file(path: &std::path::Path) {
    match std::fs::remove_file(path) {
        Ok(()) => info!("removed {}", path.display()),
        Err(e) if e.kind() == io::ErrorKind::NotFound => {}
        Err(e) => warn!("failed to remove {}: {e}", path.display()),
    }
}

fn bin_path() -> io::Result<PathBuf> {
    Ok(home_dir()?.join(".local/bin/stickshift"))
}

#[cfg(target_os = "linux")]
pub fn uninstall() -> io::Result<()> {
    match Command::new("systemctl")
        .args(["--user", "disable", "--now", "stickshift"])
        .status()
    {
        Ok(s) if s.success() => info!("stopped and disabled the stickshift service"),
        Ok(s) => warn!("systemctl disable exited with {s} (service may not have been installed)"),
        Err(e) => warn!("failed to run systemctl: {e}"),
    }

    remove_file(&home_dir()?.join(".config/systemd/user/stickshift.service"));

    if let Err(e) = Command::new("systemctl").args(["--user", "daemon-reload"]).status() {
        warn!("failed to run systemctl daemon-reload: {e}");
    }

    remove_file(&bin_path()?);

    Ok(())
}

#[cfg(target_os = "macos")]
pub fn uninstall() -> io::Result<()> {
    let plist_path = home_dir()?.join("Library/LaunchAgents/adiSuper94.stickshift.plist");
    let _ = Command::new("launchctl")
        .arg("unload")
        .arg(&plist_path)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    remove_file(&plist_path);
    remove_file(&bin_path()?);

    Ok(())
}
