# Action script cookbook (MacOS)

Reference snippets for writing your own action scripts (the ones wired up
under `~/.config/stickshift/actions/{in,out}/<gear>.sh`). Copy whichever
snippet you need into your own script.

## Opening files and URLs (`open`)

The `open` command is used to open files, or directory in the default app.

```sh
open "$HOME/Downloads/somefile.pdf"
```

The `open` command can also be used to open URLs in the default web browser.

```sh
open "https://google.com"
```

Open a file with a specific application instead of the default:

```sh
open -a "Preview" "$HOME/Documents/receipt.pdf"
```

Reveal a file in Finder instead of opening it:

```sh
open -R "$HOME/Documents/notes.txt"
```

## Controlling macOS apps (`osascript` / AppleScript)

Open (or switch to) an application:

```sh
osascript -e 'tell application "Claude" to reopen'
```

Quit an application gracefully

```sh
osascript -e 'tell application "Claude" to quit'
```

Create a toast notification:

```sh
osascript -e 'display notification "Hello world!" with title "My App"'
```

Say something out loud:

```sh
say "I am Batman!"
```

Make an application's front window full screen. Requires granting Accessibility
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

## Closing multiple apps at once

same script as the single-app `osascript` "quit", but in a loop for multiple apps:

```sh
for app in Brave Claude Calendar; do
    osascript -e "tell application \"$app\" to quit"
done
```
