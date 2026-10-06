#!/usr/bin/env bash
# Repair only this installed source; never resolve a remote tag or replace code.
set -Eeuo pipefail
root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
plugin_id='io.github.tcballard.familiar-desktop'
installed="$HOME/.config/omarchy/plugins/$plugin_id"
style="${1:-windows}"
[[ $# -le 1 && ( "$style" == windows || "$style" == mac ) ]] || { echo 'Usage: bash repair.sh [windows|mac]' >&2; exit 1; }
[[ -d "$installed" && "$(cd -- "$installed" && pwd -P)" == "$root" ]] || { echo 'Run repair from the installed Familiar checkout.' >&2; exit 1; }
[[ -d "$root/.git" && ! -L "$installed" ]] || { echo 'Repair requires the installed Git checkout.' >&2; exit 1; }
[[ -z "$(git -C "$root" status --porcelain --untracked-files=normal)" ]] || { echo 'Local changes found; repair will not overwrite them.' >&2; exit 1; }
# Versioned installers check ABI/checksums and replace only owned binary files.
bash "$root/install-titlebars.sh" --check
if [[ -x "$root/bin/familiar-desktop" ]]; then
  "$root/bin/familiar-desktop" desktop restore
  "$root/bin/familiar-desktop" titlebars disable
fi
omarchy plugin disable "$plugin_id"
trap 'echo "Repair stopped; complete repair before enabling Familiar." >&2' ERR
bash "$root/install-backend.sh"
library="$(bash "$root/install-titlebars.sh")"
"$root/bin/familiar-desktop" titlebars setup --library "$library" --enable --style "$style"
omarchy plugin enable "$plugin_id"
omarchy-shell "$plugin_id" refresh
omarchy-shell "$plugin_id" refreshTitlebars
