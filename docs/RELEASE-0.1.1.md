# v0.1.1 preparation and release gate

Base: f61093271346f57447b39b22ac9fd4d01fb061d9. Do not publish or tag until owner acceptance and final CI.

## Required XPS checks

- Upgrade from v0.1.0: preserve dock pins, layout, theme, Caps Lock and titlebar choices.
- Windows settings initially use existing configuration. Enable Floating explicitly, then press Super+Enter repeatedly. Existing tiles must retain size; new terminals must float. Repeat with another app and a dialog. No Hyprland config errors.
- Remove the preference, open another terminal and confirm normal tiling. Existing floaters stay floating until explicitly returned to tiling. Uninstall removes only Familiar's owned include and separate generated file; retain unrelated edits. Edited owned files must stop cleanup.
- Minimise via titlebar and app menu; confirm app remains in dock and restores on current workspace. Test multiple named windows, other workspaces, two monitors, negative monitor coordinates and mixed scaling.
- Activate each named window from the context menu and by dock click. Verify exact window focus and pointer centre after animation, without focus bouncing to the dock. Minimising must not unexpectedly warp to another window. Close a window while its menu is open.
- Test Settings repair with complete, missing-backend and failed-download cases. It must not change checkout SHA or resolve tags; failures leave a clear retry instruction. Candidate repair requires published assets of the matching version, so do not use Settings repair before publication.
- Review and capture the actual current desktop. The uploaded teaser is an illustrative animation, not live evidence. Its v0.1.0 Coming soon end card must change before a v0.1.1 launch.

## Release procedure

Run required portable/Rust/QML/install/update/remove checks and release preflight. Merge the accepted candidate through protected main, wait for exact-main Portable checks and Release binaries, then create the annotated v0.1.1 tag via Tag verified release. Do not move v0.1.0.

The repository install.sh is a template with @SOURCE_SHA@ and deliberately refuses execution. scripts/prepare-release.cjs emits install.sh with the exact built source SHA, includes it in the release manifest and SHA256SUMS, and emits the test bundle's pinned install-candidate.sh. Inspect these pins against the final main SHA before publication. Obtain source provenance from the immutable reviewed commit; same-release checksums are not independent signatures.

After publishing and verifying all downloads, versions, checksums, manifests and supported ABI, update the README installation command to the new bootstrap at an immutable source location (or explicitly document the trust of downloading the release asset). Update marketplace issue 10184 to the final reviewed source. No publication or marketplace update is performed by this preparation PR.
