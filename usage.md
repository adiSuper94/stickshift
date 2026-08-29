# Action script cookbook

Reference snippets for writing your own action scripts (the ones wired up
under `~/.config/stickshift/actions/{in,out}/<gear>.sh`). Copy whichever
snippet you need into your own script.

## Opening files and URLs (`open` / `xdg-open`)

`open` is macOS's command for this; `xdg-open` is the Linux equivalent.
This helper picks whichever one is actually present, so the same call
works on both platforms:

```sh
open_or_xdg_open() {
    if command -v open >/dev/null 2>&1; then
        open "$1"
    elif command -v xdg-open >/dev/null 2>&1; then
        xdg-open "$1"
    else
        echo "No 'open' or 'xdg-open' command found on this system." >&2
        return 1
    fi
}
```

Open a URL in the default browser:

```sh
open_or_xdg_open "https://elgato.com"
```

Open a file with its default application:

```sh
open_or_xdg_open "$HOME/Documents/notes.txt"
```

Real example — the UF payable-time approval page:

```sh
open_or_xdg_open "https://my.ufl.edu/psp/ps/EMPLOYEE/HRMS/c/ROLE_MANAGER.TL_SRCH_APPRV_GRP.GBL?NAVSTACK=Clear&cmd=uninav&Rnode=HRMS&uninavpath=Root%7bPORTAL_ROOT_OBJECT%7d.NO_CRUMB%7bPTUN_11453933000065419%2cPORTAL_ROOT_OBJECT%7d.Approve%20Payable%20Time%7bHC_TL_SRCH_APPRV_GRP_GBL%7d"
```

macOS only — open a file with a specific application instead of the default:

```sh
open -a "Preview" "$HOME/Documents/receipt.pdf"
```

macOS only — reveal a file in Finder instead of opening it:

```sh
open -R "$HOME/Documents/notes.txt"
```

## Controlling macOS apps (`osascript` / AppleScript)

macOS only — `osascript` doesn't exist on Linux.

Open (or switch to) an application:

```sh
osascript -e 'tell application "Safari" to activate'
```

Quit an application gracefully (same as Cmd+Q) — lets the app run its own
shutdown code (prompt to save, clean up, etc.), unlike a raw
`pkill`/SIGTERM which the app may not specially handle:

```sh
osascript -e 'tell application "Safari" to quit'
```

Make an application's front window full screen. There's no plain "full
screen" verb in most apps' AppleScript dictionaries, so this goes through
System Events' accessibility API instead, toggling the same `AXFullScreen`
attribute the green button does. Requires granting Accessibility
permission to whatever runs this script, under System Settings > Privacy
& Security > Accessibility:

```sh
osascript -e '
    tell application "Safari" to activate
    tell application "System Events"
        tell process "Safari"
            set value of attribute "AXFullScreen" of window 1 to true
        end tell
    end tell'
```

## Closing multiple apps at once ("shutdown mode")

Portable across macOS and Linux via `pkill` (sends `SIGTERM`, not
`SIGKILL`). Caveat: this is only a best-effort approximation of a graceful
quit — it behaves gracefully only if the target app actually handles
`SIGTERM`, which isn't guaranteed. For an actually-graceful quit on macOS,
use the per-app `osascript` "quit" snippet above instead.

```sh
for app in Todoist Claude Calendar; do
    pkill -x "$app" 2>/dev/null
done
```
