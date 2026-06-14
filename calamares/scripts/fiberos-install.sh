#!/usr/bin/env bash

set -euo pipefail

target_root="${1:-/mnt}"
install_source="${FIBER_INSTALL_SOURCE:-/run/fiberos/rootfs}"

if [[ ! -d "$target_root" ]]; then
	echo "target root does not exist: $target_root" >&2
	exit 1
fi

if [[ ! -d "$install_source" ]]; then
	echo "install source does not exist: $install_source" >&2
	echo "set FIBER_INSTALL_SOURCE to the staged Fiber rootfs path" >&2
	exit 1
fi

command -v rsync >/dev/null 2>&1 || {
	echo "rsync is required by the Fiber installer" >&2
	exit 1
}

rsync -aHAX --delete \
	--exclude=/dev \
	--exclude=/proc \
	--exclude=/sys \
	--exclude=/run \
	--exclude=/tmp \
	--exclude=/mnt \
	"$install_source"/ "$target_root"/

mkdir -p "$target_root/dev" "$target_root/proc" "$target_root/sys" "$target_root/run" "$target_root/tmp" "$target_root/mnt"
chmod 1777 "$target_root/tmp"

if [[ -x "$target_root/usr/lib/fiberos/install-bootloader" ]]; then
	"$target_root/usr/lib/fiberos/install-bootloader" "$target_root"
else
	echo "warning: Fiber bootloader installer not found; skipping bootloader installation" >&2
fi
