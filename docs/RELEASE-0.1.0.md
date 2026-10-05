# Familiar v0.1.0 release preparation

Prepared on Monday 5 October 2026 (UK); formal publication is deferred. This document prepares the release; it does not schedule publication or claim the release exists.

## Frozen scope

Core desktop only: dock and named windows; three layout presets; Automatic/Bottom/Left/Right dock placement; optional title bars; centred settings; window arrangement and recovery; Show Desktop/Restore; dock/title-bar sizes; Quit/confirmed Force Quit; active shortcuts; optional file shortcuts; explicit Caps Lock/Compose preference.

Getting Started contains System settings and Troubleshooting. The v0.2.0 app collection will expose Open for installed apps and Install only after a supported package is verified. No Paint, Notepad, Task Manager, OmaStore or Postcard installation is part of v0.1.0. These apps are not release dependencies.

## Candidate and version

The manifest, Cargo package/lockfile and all three release installers declare 0.1.0. The release executable test derives its expected version from the manifest to avoid another stale rc assertion. Historical rc.1/rc.2 notes remain historical.

Until the tag/assets exist, use the `familiar-desktop-release` artifact from the successful **Release binaries** run for this exact candidate. Extract it and run:

```bash
bash install-candidate.sh windows
```

Use `mac` for left-side controls. This installer embeds the full source SHA and version. Do not run the unexpanded template in the source tree or the future v0.1.0 download installer before assets are published. The README links the immutable published rc.4 ZIP. Its source is `28e192a2f6f733b37b49f7e524290fd4234004c7`; its ZIP SHA-256 is `f7d5fd7593f42bd68688739376860c513cc4198036bfd7b3b1bccc8e4fa7d9f9`. Main includes later installer and workflow preparation, so rc.4 testing alone does not validate those changes.

## Before publication

- [ ] Merge the preparation PR after Portable checks and Release binaries pass. Record the resulting main commit and its own successful CI runs.
- [ ] Inspect the exact candidate bundle: static executable version 0.1.0, supported Hyprbars ABI, complete SHA256SUMS, release/source manifests, SPDX document, notices, installer and XPS guide.
- [ ] Run `omarchy plugin validate` on the candidate on XPS and record its output. The portable skill preflight was run during the rc.4 review; it does not substitute for the installed Omarchy validator or live checks.
- [ ] Complete docs/XPS-TEST.md on that exact source. Record `omarchy-version`, `hyprctl version`, monitor/scale, plugin SHA and each result.
- [ ] Check fresh install, upgrade from v0.0.6, repeat install, settings/pin retention, local-change refusal, disable/re-enable, Caps reset, title-bar removal and rollback. Preserve personal files; use disposable test profiles where possible.
- [ ] Verify actual window actions, Show Desktop/Restore, focus/dismissal, keyboard behavior after reload/login, themes/scaling and available monitor coverage. State unavailable hardware coverage explicitly.
- [ ] Complete docs/ROLLBACK.md: separate settings, exact fresh-install cleanup, later-edit preservation, refusal paths, live keyboard/title-bar reset and the installed uninstall command.
- [ ] Finish the compatibility paragraph in docs/v0.1.0.md using those results. Replace the rendered settings preview with a real screenshot when available.
- [ ] Review the existing release download trust boundary before public promotion: checksums detect corruption but are fetched beside the release; immutable provenance binding and workflow dependency pinning are not established by these checks. Do not describe this as a security audit.
- [ ] Freeze source, create an annotated v0.1.0 tag at that exact commit, and publish the release using the approved notes. Never move the tag. Observe Release binaries through asset upload and verify the downloaded published assets.
- [ ] Switch the README's primary install/update link from rc.4 to v0.1.0 only once those assets are present. Keep the rollback instructions.

The live checks and final provenance/publication gates are outstanding. A green portable build is a testable candidate, not a release sign-off. Do not cut the tag or publish just because the planned date has arrived.

## Evidence record

Base at preparation start: `90db20ac1919486da3fc7e55387ecfc30188bb6d` (rc.2). Both main workflows passed. This branch changes core scope presentation, version alignment, release documentation and release-version assertions. Fresh CI results belong to this PR and must not be replaced with the rc.2 results.

The prepared bundle includes this checklist and the release-note draft, with their hashes in the release manifest and SHA256SUMS. XPS acceptance uses the guide inside that same bundle.

## Final preparation changes

The main preparation consolidates the settings, dock placement, cleanup and both Caps Lock fixes. The release installer now checks compatibility before registration, refuses unmanaged/dirty/untracked/unexpected ignored files, restores v0.1.0 windows, disables owned controls and the plugin before checkout, and repeats the clean-tree check before installing. Regression fixtures cover updates from v0.0.6 and v0.1.0 plus refusal and recovery failures.

Release CI accepts exact final tags and numbered `vX.Y.Z-rc.N` tags marked as prereleases. It checks out the event tag explicitly. Prereleases build and validate artifacts; only a final release automatically uploads the normal installer assets. Existing published tags and ZIPs remain unchanged. The old rc.2–rc.4 release-event runs failed the old exact-tag check; their ZIPs came from successful PR builds. New workflow code cannot retroactively change those historical runs.

The maintainer reported rc.4 looked good on the desktop. This is useful feedback, not a completed per-step acceptance record. The formal tag, release and source-bound live checks above remain pending. Bug and feature-request forms are available for incoming feedback.
