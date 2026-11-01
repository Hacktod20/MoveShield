#!/usr/bin/env bash
set -euo pipefail

archive=${1:?usage: smoke-linux-install.sh ARCHIVE SHA256SUMS PROJECT_ROOT VERSION}
checksums=${2:?usage: smoke-linux-install.sh ARCHIVE SHA256SUMS PROJECT_ROOT VERSION}
project_root=${3:?usage: smoke-linux-install.sh ARCHIVE SHA256SUMS PROJECT_ROOT VERSION}
version=${4:?usage: smoke-linux-install.sh ARCHIVE SHA256SUMS PROJECT_ROOT VERSION}
archive_name=$(basename "$archive")
temporary_root=$(mktemp -d)
trap 'rm -rf "$temporary_root"' EXIT

download_directory="$temporary_root/download"
install_directory="$temporary_root/home/.local/bin"
mkdir -p "$download_directory" "$install_directory"
cp "$archive" "$checksums" "$download_directory/"

grep -F "  $archive_name" "$download_directory/SHA256SUMS" |
    (cd "$download_directory" && sha256sum --check)
tar -xzf "$download_directory/$archive_name" -C "$install_directory"
test -x "$install_directory/moveshield"

export PATH="$install_directory:$PATH"
test "$(command -v moveshield)" = "$install_directory/moveshield"
test "$(moveshield --version)" = "moveshield $version"

analysis_status=0
moveshield analyze "$project_root/apps/sui-cli/tests/fixtures/no_workspace_mutation" --format json \
    > "$temporary_root/report.json" || analysis_status=$?
test "$analysis_status" -eq 0
test -s "$temporary_root/report.json"
grep -F '"project": "no_workspace_mutation"' "$temporary_root/report.json"

printf '#!/usr/bin/env sh\nprintf "old moveshield\n"\n' > "$install_directory/moveshield"
chmod 755 "$install_directory/moveshield"
test "$(moveshield --version)" = 'old moveshield'
tar -xzf "$download_directory/$archive_name" -C "$install_directory"
test -x "$install_directory/moveshield"
test "$(moveshield --version)" = "moveshield $version"

rm "$install_directory/moveshield"
test ! -e "$install_directory/moveshield"
! command -v moveshield >/dev/null 2>&1
printf '%s\n' 'Linux Sui compiler release install, upgrade, and uninstall smoke test passed.'