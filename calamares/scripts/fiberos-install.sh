#!/usr/bin/env bash

set -euo pipefail

target_root="${1:-/mnt}"
install_source="${FIBER_INSTALL_SOURCE:-/run/fiberos/rootfs}"
fiber_profile="User"

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
