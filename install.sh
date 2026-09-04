#!/usr/bin/env bash
# Downloads the latest stickshift release and installs it as a background service
set -euo pipefail

REPO="adiSuper94/stickshift"
BIN_DIR="$HOME/.local/bin"
BIN_PATH="$BIN_DIR/stickshift"

os="$(uname -s)"
arch="$(uname -m)"

case "$os" in
  Linux)
    case "$arch" in
      x86_64) asset="stickshift-x86_64-linux" ;;
      aarch64 | arm64) asset="stickshift-aarch64-linux" ;;
      *)
        echo "Unsupported Linux architecture: $arch" >&2
        exit 1
        ;;
    esac
    ;;
  Darwin)
    case "$arch" in
      arm64 | aarch64) asset="stickshift-aarch64-darwin" ;;
      *)
        echo "Unsupported macOS architecture: $arch (only Apple Silicon builds are published)" >&2
        exit 1
        ;;
    esac
    ;;
  *)
    echo "Unsupported OS: $os" >&2
    exit 1
    ;;
esac

echo "Downloading latest stickshift ($asset)..."
mkdir -p "$BIN_DIR"
tmp="$(mktemp)"
curl -fL "https://github.com/$REPO/releases/latest/download/$asset" -o "$tmp"
chmod +x "$tmp"

if [ "$os" = "Linux" ]; then
  systemctl --user stop stickshift >/dev/null 2>&1 || true
else
  launchctl unload "$HOME/Library/LaunchAgents/adiSuper94.stickshift.plist" >/dev/null 2>&1 || true
fi

mv "$tmp" "$BIN_PATH"
echo "Installed $BIN_PATH"

if [ "$os" = "Linux" ]; then
  unit_dir="$HOME/.config/systemd/user"
  mkdir -p "$unit_dir"
  cat >"$unit_dir/stickshift.service" <<EOF
[Unit]
Description=stickshift

[Service]
ExecStart=$BIN_PATH
Restart=on-failure

[Install]
WantedBy=default.target
EOF
  systemctl --user daemon-reload
  systemctl --user enable --now stickshift
  echo "stickshift is installed and running as a systemd user service."
  echo "Check status:  systemctl --user status stickshift"
  echo "Tail logs:     journalctl --user -u stickshift -f"
else
  agents_dir="$HOME/Library/LaunchAgents"
  mkdir -p "$agents_dir"
  plist="$agents_dir/adiSuper94.stickshift.plist"
  cat >"$plist" <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/dtds/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>Label</key>
  <string>adiSuper94.stickshift</string>
  <key>ProgramArguments</key>
  <array>
    <string>$BIN_PATH</string>
  </array>
  <key>RunAtLoad</key>
  <true/>
  <key>KeepAlive</key>
  <true/>
</dict>
</plist>
EOF
  launchctl load "$plist"
  echo "stickshift is installed and running as a launchd agent."
  echo "Check status:  launchctl list | grep stickshift"
fi

case ":$PATH:" in
  *":$BIN_DIR:"*) ;;
  *) echo "Note: $BIN_DIR is not on your PATH." ;;
esac
