# Installer source identity

## v0.1.1 candidate

The repository `install.sh` is now an unexpanded template: it refuses to run until release CI inserts the exact built 40-character SHA into the standalone installer asset. The candidate bundle uses the same exact SHA. Settings copies a command for `repair.sh`, which repairs the installed source without any remote Git fetch or checkout. Neither path silently resolves a moving source tag. See [release gates](RELEASE-0.1.1.md).

The following records the historical v0.1.0 bootstrap fix, which remains the stable README command until v0.1.1 is published.

## v0.1.0 bootstrap history

The hardened bootstrap at commit `57b6fc07b2221357f7d93e031940123e41f37b2c` installs the published v0.1.0 desktop
at full commit `bda1ec617966b11fb8470788c74019350b38838f`. This is a separate
bootstrap change after the release; it does not move v0.1.0 or replace its assets.
Use the commit-pinned bootstrap command in the current README.

The bootstrap fetches only that commit, checks the fetched commit identity,
checks out detached, and verifies HEAD before running its installation scripts.
A fresh install is staged and verified before `omarchy plugin add` receives the
local repository. The installed origin is then restored to the public upstream.
This avoids registering the remote default branch before pinning. Updates still
restore windows and unload owned controls before changing the checkout.

The CI candidate installer follows the same staging boundary, using the full
SHA embedded in its built bundle. The Hyprbars CI build already fetches and
verifies full upstream commit `7644cecdb947060682891a0db2a0cdc5c0b9e704` before
building it; it is not compiled on the user's desktop.

## Limits and release identity

The published v0.1.0 checkout still contains its original installer, including
the original Settings repair action. Existing copies and historical release
instructions cannot be retroactively changed. Use the current README bootstrap
command for hardened installation and repair; the old Settings repair action
is not covered by this change. A future desktop release must carry the updated
installer and its own reviewed source pin.

Source pinning does not authenticate downloaded release binaries independently.
The existing installers verify checksums downloaded from the same GitHub release
and check the backend version. They continue to trust the publisher's release
assets. Repository tag rules prevent tag updates and deletion, but are not
independent signatures and do not make release assets immutable.

The marketplace review must distinguish this bootstrap revision from the
installed desktop SHA above. Passing tests or repository rules do not grant
marketplace approval. Privilege/package-manager capabilities remain disclosed:
only missing download tools may prompt for `sudo pacman`.

## Evidence

`node tests/test_install_source.cjs` uses real local Git repositories with a
reviewed commit followed by an unreviewed default-branch commit and a moved
version tag. Fresh and upgrade installations remain at the reviewed commit.
Injected fetch and checkout identity mismatches stop before downloaded install
scripts or plugin enablement. Existing installer and candidate lifecycle tests
cover checksum, ABI, dirty checkout, restore/unload and setup failures.
These are automated fixture tests, not a new live Omarchy desktop test.
