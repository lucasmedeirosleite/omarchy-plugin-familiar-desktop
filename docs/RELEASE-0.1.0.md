# Familiar v0.1.0 release record

## Scope and owner acceptance

The release includes the dock, named windows, General/Windows/Mac layouts, Automatic/Bottom/Left/Right placement, optional window controls, centred settings, window arrangement and recovery, Show Desktop/Restore, sizes, Quit/confirmed Force Quit, active shortcuts, file shortcuts and Caps Lock/Compose preference. The app collection stays planned for v0.2.0.

On 5 October 2026 the maintainer approved rc.5 after XPS testing: “Okay this looks solid to me now. We should mark v0.1.0 and do the submission!” The tested source is `377f8823c1b8a013131d75d614339bf569c9a68e`. The final preparation changes release documentation and adds an explicit annotated-tag workflow; runtime and installer source remain identical to rc.5.

## Automated evidence

Both main workflows passed for rc.5: Portable checks run 37359006140 and Release binaries run 37359006230. Coverage includes Rust tests and Clippy, Lua 5.4/5.5 Caps Lock regressions, QML tests/parsing, 48 dock placement combinations, settings persistence, 12 original installer scenarios plus compatibility/asset failures, 14 update lifecycle scenarios, 11 candidate installer scenarios, 7 uninstall scenarios and final/RC tag validation. Clean-tree release preflight also passed with advisory capability findings requiring review; this is not a security audit.

Final documentation/tag preparation receives its own CI runs. The manually invoked **Tag verified release** workflow requires the current main SHA and successful Portable checks and Release binaries runs before making an annotated version tag. It refuses to replace an existing tag. Final publication runs Release binaries again and uploads immutable versioned assets.

## Artifact and source identity

The published release and tag identify the final full SHA. `RELEASE-MANIFEST.json` and `SOURCE-MANIFEST.json` bind the executable, source archive, Hyprbars library, documents and notices to that SHA. `SHA256SUMS` covers the assets. The rc.5 ZIP remains immutable and available separately, with SHA-256 `86831e8b43183cf2fc86b9b29235de44f8b10bb64cb9459b4b72304128978916`.

## Compatibility and remaining coverage

Supported prebuilt target: Linux x86_64, Omarchy Quattro, Hyprland 0.56.2 commit `efb50993780079460b0cbed1363e2166a2de1d9f`, ABI `efb50993780079460b0cbed1363e2166a2de1d9f_aq_0.15_hu_0.14_hg_0.5_hc_0.1_hlg_0.6`.

The owner supplied overall desktop acceptance. An exact installed Omarchy revision, per-step acceptance log, exhaustive app/monitor/scaling results and a fresh-profile lifecycle record were not supplied. The formal download installer has fixture coverage; the owner's candidate installation used the candidate bundle route. Keep these limits explicit when responding to feedback or marketplace review. A rendered preview is labelled as such in the README.

## Installation and removal

The versioned installer uses public GitHub downloads, SHA-256 and executable version checks. It may install missing download tools with pacman after a privilege prompt. It never builds on the user's machine. Unsupported ABIs and local-file conflicts stop setup. See README for installation and `uninstall.sh` for owned-configuration cleanup; settings, pins and backups are retained.

Checksums downloaded beside assets detect corruption; they are not independent publisher attestations. Existing build workflow dependencies are not all pinned to immutable identities. Marketplace approval remains a separate exact-commit review.
