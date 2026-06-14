#!/usr/bin/env bash

set -euo pipefail

target_root="${1:-/mnt}"
selected_profile="${FIBER_PROFILE:-User}"

if [[ ! -d "$target_root" ]]; then
	echo "target root does not exist: $target_root" >&2
	exit 1
fi

case "$selected_profile" in
	User | Developer | Hacker)
		;;
	*)
		echo "unsupported Fiber profile: $selected_profile" >&2
		exit 1
		;;
esac

mkdir -p "$target_root/etc/fiberos"

cat > "$target_root/etc/fiberos/install.conf" <<EOF
FIBER_INSTALLER=calamares
FIBER_INSTALL_MODE=custom-image
FIBER_PROFILE=$selected_profile
EOF

cat > "$target_root/etc/fiberos/README" <<'EOF'
Fiber OS installer state

The installer staged Fiber OS using the custom image pipeline.
Bootloader and recovery configuration are owned by Fiber installer scripts.
EOF
