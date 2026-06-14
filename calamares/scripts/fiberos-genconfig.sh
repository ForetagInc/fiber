#!/usr/bin/env bash

set -euo pipefail

target_root="${1:-/mnt}"

if [[ ! -d "$target_root" ]]; then
	echo "target root does not exist: $target_root" >&2
	exit 1
fi

mkdir -p "$target_root/etc/fiberos"

cat > "$target_root/etc/fiberos/install.conf" <<'EOF'
FIBER_INSTALLER=calamares
FIBER_INSTALL_MODE=custom-image
EOF

cat > "$target_root/etc/fiberos/README" <<'EOF'
Fiber OS installer state

The installer staged Fiber OS using the custom image pipeline.
Bootloader and recovery configuration are owned by Fiber installer scripts.
EOF
