# Separate configuration and removal

Familiar keeps its settings and generated configuration separate. Guarded
includes let Hyprland load optional title bars, the explicit keyboard preference
and the opt-in floating-window preference on login/reload. It does not replace your config
with a complete preset.

| Feature | Familiar-owned data | Personal config integration |
| --- | --- | --- |
| Dock, pins, placement | `~/.config/omarchy/familiar-desktop-*.json` | No Hyprland edits |
| Title bars | `~/.config/omarchy/familiar-titlebars/titlebars.lua` and ownership record | Marked include in `hypr/looknfeel.lua` |
| New floating windows | `~/.config/omarchy/familiar-windows/window-mode.lua` | Marked include at the end of `hypr/hyprland.lua` |
| Caps Lock/Compose | `~/.config/omarchy/familiar-input/caps-lock.lua` | Marked include at the end of `hypr/hyprland.lua` |

The Rust helpers respect `XDG_CONFIG_HOME` for the two Hyprland files and their
generated Lua. The dock files and Omarchy plugin installation use the default
home-based locations. Changing Caps Lock choices updates the separate Lua file;
it does not rewrite the include. Choosing **Use configuration** removes the
include and generated keyboard config. No keyboard override is installed merely
by enabling Familiar.

## Remove the v0.1.1 candidate

Run the script from the installed candidate, before deleting its folder:

```bash
bash ~/.config/omarchy/plugins/io.github.tcballard.familiar-desktop/uninstall.sh
```

It restores Show Desktop windows recorded for the current session, refuses to
continue if hidden/minimised windows remain, disables Familiar, resets the
keyboard and floating-window overrides, removes the owned title-bar include, explicitly unloads
Hyprbars, and checks the reload before asking Omarchy to delete the plugin.
If it asks you to restore minimised windows, use the dock and rerun the command.
The shared `special:minimized` workspace does not identify who minimised each
window, so the uninstaller does not guess their ownership or original workspace.

Any failure stops deletion. The plugin may be disabled and earlier cleanup steps
may already be complete; fix the reported error and rerun. Cleanup is repeatable.
An edited, duplicated, foreign or unowned hook is refused. A symlinked personal
config is refused. Detected concurrent edits are retained. Review backups and
merge manually rather than blindly replacing your whole config.

The generic **omarchy plugin remove** command has no plugin cleanup callback.
It is not a substitute for this script. If the folder was already removed,
reinstall the same candidate before cleanup. Missing manifests make generated
overrides inert on the next Hyprland reload; that is not immediate removal of
an already loaded plugin or restoration of hidden windows.

## What is preserved

New title-bar installs append an exact removable block without trimming original
bytes. Removal preserves those bytes, permissions and later edits outside the
block. Earlier builds trimmed trailing whitespace: migration preserves the file
as it now exists and cannot reconstruct whitespace already lost. Legacy inline
Caps Lock blocks migrate when a preference is next applied, or can be reset
directly.

Preferences, pins, window recovery records and private recovery backups remain.
Title-bar backups are in `omarchy/familiar-titlebars/backups` under the config
directory; keyboard backups are in `omarchy/familiar-caps-lock` under the XDG
state directory. Removing Familiar does not uninstall unrelated packages or a
Hyprbars installation managed elsewhere. Its bundled library is removed with
the plugin folder. This is targeted cleanup, not a snapshot of all desktop state
or a promise to restore window tiling order.

## Acceptance on XPS

On a disposable profile, record both personal config files and their modes before
installation, including a file without a final newline. Install, select Normal
Caps Lock then Compose, change title-bar style and dock position, and verify that
only Familiar-owned blocks differ. Add an unrelated personal setting after
installation, uninstall, and compare: that edit must remain and all enabled hooks must
be gone. Check `hyprctl configerrors`, `hyprctl plugin list`, keyboard behavior and
window visibility. Repeat removal and exercise a deliberately edited hook: it
must stop without deleting the plugin. Portable fixtures do not prove live
Hyprland rollback.

Floating mode is opt-in and applies only when new windows open. Reset/uninstall removes the rule and preserves unrelated configuration; it does not retile windows already floating. Its recovery backups live under `omarchy/familiar-window-mode` in the XDG state directory.

## Trackpad preferences (after rc.1)

Run the installed `bin/familiar-desktop gestures reset` or choose Input → Trackpad
→ Use configuration. Familiar removes only its exact generated
`~/.config/omarchy/familiar-trackpad/gestures.lua` and guarded include in
`~/.config/hypr/hyprland.lua`. Backups live under
`~/.local/state/omarchy/familiar-gestures` (respecting XDG overrides). Edited hooks,
symlinks and failed reloads are refused or rolled back. Uninstall resets gestures
before removing the plugin; disabling the shell plugin alone retains preferences.
