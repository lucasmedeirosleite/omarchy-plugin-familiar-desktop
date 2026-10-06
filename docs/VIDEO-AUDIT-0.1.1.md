# Teaser audit for v0.1.1

Input: familiar-teaser-16x9-coming-soon(1).mp4, 20 seconds. Reviewed sampled frames at 0,2,...18 seconds and source paths at the candidate based on f610932.

| Scene | Implementation / qualification |
| --- | --- |
| Find the right window (around 6s) | components/AppMenu.qml lists named windows. Clicking a row calls restoreOrLaunchItem with the exact address. This selects an existing window; it does not create a new terminal with that title. New Window is a separate launch action. |
| Pointer moves into selected window | v0.1.0 only requests focus. This candidate explicitly centres it after restore/focus. Needs live mixed-monitor/scale verification. |
| Make room (around 8s) | Show Desktop / Restore are implemented using a journal and a hidden workspace. This is separate from minimising an individual window. |
| Choose what feels familiar (around 10s) | General / Windows / Mac are preference presets, not full replicas of those operating systems. |
| Keep controls close (around 12s) | Dock controls and named-window actions exist. Exact composition and theme depend on user settings. |
| Caps Lock (around 14s) | Explicit normal/Compose preference exists. It does not silently override per-device configuration. |
| Real screenshot (around 16s) | Historical image, not proof that this candidate passed desktop tests. |
| End card (around 18s) | Says v0.1.0 Coming soon. Needs v0.1.1 and publication-appropriate wording before launch. |

The animated terminal split in the opening illustrates tiling. Floating preference is new, opt-in and does not retroactively rearrange existing windows. Do not describe it as a never-shrinking automatic cascade. No video pixels were altered during this audit.
