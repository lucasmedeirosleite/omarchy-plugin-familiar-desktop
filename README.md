# Familiar Desktop for Omarchy

A mouse-friendly way to launch apps, see what is open, and choose the exact
window you want. The same dock has three starting layouts: **General** for
ordinary day-to-day use, **Windows** for a persistent bottom app strip, and
**Mac** for a bottom dock that reveals on hover. All three share the same
pinned apps, folders, window matching, and settings.

This is a development prototype. It is derived from
[rosakodu/omarchy-dock](https://github.com/rosakodu/omarchy-dock) at commit
`467070386fe60e173295020d3911176202b3e0c9` under the MIT license.
The source is retained so its existing monitor, window, widget, and theme
handling can be tested as the product evolves. It has a distinct plugin ID and
configuration paths; it can be removed without changing the original dock's
files. A live Omarchy test is required before release.

## First slice

- Left-click launches or switches to an app; middle-click opens a new window.
- Right-click an app opens a menu of **named windows**, with an active marker,
  plus New Window, Pin/Unpin, Minimize Current Window, and Close Current Window.
  The list scrolls when an app has more than eight windows.
- The bar widget offers General, Windows, and Mac presets. Applying a preset
  changes visibility and overlay behavior. Windows and Mac prefer the bottom
  edge unless an existing bottom bar would overlap; General follows the bar's
  opposite edge. Subsequent manual changes to the controls remain in effect
  until another preset is chosen.
- Folder, widget, badge, multi-monitor, and workspace behavior from the
  original implementation remains available for evaluation.

| Preset | Edge | Visibility | Window layout |
| --- | --- | --- | --- |
| General | Opposite the bar | Always shown | Reserves space |
| Windows | Bottom when free | Always shown | Reserves space |
| Mac | Bottom when free | Reveal on hover | Overlays windows |

These are starting layouts, not a claim of complete Windows or macOS behavior.
The Familiar theme, Task Manager, and OmaStore are separate optional projects;
this shell plugin does not install or change them.

## Development checkout

On an Omarchy Quattro machine, validate the checkout first:

```bash
omarchy plugin validate ./familiar-desktop
```

Then install from a Git repository containing this folder at its root, using
`omarchy plugin add <repository-url> --enable`. The permanent plugin ID is
`io.github.tcballard.familiar-desktop`. Do not install the local archive as if
it were a Git repository. Use the bar's **Familiar Desktop** widget to select a
preset; it can also be changed through IPC:

```bash
omarchy-shell io.github.tcballard.familiar-desktop setProfile general
omarchy-shell io.github.tcballard.familiar-desktop setProfile windows
omarchy-shell io.github.tcballard.familiar-desktop setProfile mac
```

Settings and pins are stored in `~/.config/omarchy/familiar-desktop-settings.json`
and `~/.config/omarchy/familiar-desktop-pinned.json`. Removing this plugin does
not remove those user preferences or edit the original dock's files. The two
docks should not be enabled together because they would occupy the same edge.

## Verification needed on Omarchy

Test initial loading, preset switching, window menus, a minimized window,
bar at top and bottom, two monitors, workspace switching, light/dark themes,
200% scale, shell reload, disable, and removal. In particular, verify menu
placement and dismissal against tiled and fullscreen windows before release.

## License

MIT. Original work © 2026 rosakodu; Familiar Desktop changes © 2026 Tom
Ballard. See [LICENSE](LICENSE).
