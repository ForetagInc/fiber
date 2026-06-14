#!/usr/bin/env bash

set -euo pipefail

target_root="${1:-/mnt}"
install_source="${FIBER_INSTALL_SOURCE:-/run/fiberos/rootfs}"
fiber_profile="User"
disk_encryption="luks2"

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

if [[ -f "$target_root/etc/fiberos/install.conf" ]]; then
	# shellcheck disable=SC1091
	source "$target_root/etc/fiberos/install.conf"
	fiber_profile="${FIBER_PROFILE:-User}"
	disk_encryption="${FIBER_DISK_ENCRYPTION:-luks2}"
fi

if [[ "$disk_encryption" != "luks2" ]]; then
	echo "unsupported disk encryption policy: $disk_encryption" >&2
	exit 1
fi

command -v findmnt >/dev/null 2>&1 || {
	echo "findmnt is required by the Fiber installer" >&2
	exit 1
}

command -v lsblk >/dev/null 2>&1 || {
	echo "lsblk is required by the Fiber installer" >&2
	exit 1
}

command -v cryptsetup >/dev/null 2>&1 || {
	echo "cryptsetup is required by the Fiber installer" >&2
	exit 1
}

target_source="$(findmnt -n -o SOURCE --target "$target_root" || true)"
if [[ -z "$target_source" ]]; then
	echo "could not determine target root source for encryption verification" >&2
	exit 1
fi

target_type="$(lsblk -n -o TYPE "$target_source" 2>/dev/null | head -n 1 || true)"
if [[ "$target_type" != "crypt" ]]; then
	echo "Fiber installation requires LUKS2 + dm-crypt for the root filesystem" >&2
	echo "target root source '$target_source' is type '${target_type:-unknown}', not dm-crypt" >&2
	exit 1
fi

backing_leaf="$(lsblk -n -o PKNAME "$target_source" 2>/dev/null | head -n 1 || true)"
if [[ -z "$backing_leaf" ]]; then
	echo "could not determine backing block device for '$target_source'" >&2
	exit 1
fi

backing_device="/dev/$backing_leaf"
luks_version="$(cryptsetup luksDump "$backing_device" 2>/dev/null | awk '/^Version:[[:space:]]+/ { print $2; exit }' || true)"
if [[ "$luks_version" != "2" ]]; then
	echo "Fiber installation requires LUKS2 for the root filesystem" >&2
	echo "backing device '$backing_device' reports LUKS version '${luks_version:-unknown}'" >&2
	exit 1
fi

enable_system_unit() {
	local unit="$1"
	local wanted_by="${2:-multi-user.target}"
	local unit_path=""

	for candidate in \
		"$target_root/usr/lib/systemd/system/$unit" \
		"$target_root/lib/systemd/system/$unit" \
		"$target_root/etc/systemd/system/$unit"
	do
		if [[ -f "$candidate" ]]; then
			unit_path="$candidate"
			break
		fi
	done

	[[ -n "$unit_path" ]] || return 0
	mkdir -p "$target_root/etc/systemd/system/$wanted_by.wants"
	ln -sfn "${unit_path#"$target_root"}" "$target_root/etc/systemd/system/$wanted_by.wants/$unit"
}

disable_system_unit() {
	local unit="$1"
	local wanted_by="${2:-multi-user.target}"

	rm -f "$target_root/etc/systemd/system/$wanted_by.wants/$unit"
}

case "$fiber_profile" in
	Hacker)
		enable_system_unit tor.service multi-user.target
		;;
	User | Developer)
		disable_system_unit tor.service multi-user.target
		;;
	*)
		echo "warning: unsupported Fiber profile '$fiber_profile'; leaving Tor disabled" >&2
		disable_system_unit tor.service multi-user.target
		;;
esac

if [[ -x "$target_root/usr/lib/fiberos/install-bootloader" ]]; then
	"$target_root/usr/lib/fiberos/install-bootloader" "$target_root"
else
	echo "warning: Fiber bootloader installer not found; skipping bootloader installation" >&2
fi
