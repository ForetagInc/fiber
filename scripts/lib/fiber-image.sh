#!/usr/bin/env bash

set -euo pipefail

fiber_die() {
	printf 'error: %s\n' "$*" >&2
	exit 1
}

fiber_info() {
	printf '%s\n' "$*" >&2
}

fiber_normalize_target() {
	case "${1:-}" in
		x86_64 | amd64 | x86_64-linux)
			printf 'x86_64\n'
			;;
		aarch64 | arm64 | aarch64-linux)
			printf 'aarch64\n'
			;;
		*)
			fiber_die "unsupported target '${1:-}'. Use x86_64 or aarch64"
			;;
	esac
}

fiber_host_target() {
	case "$(uname -m)" in
		x86_64 | amd64)
			printf 'x86_64\n'
			;;
		arm64 | aarch64)
			printf 'aarch64\n'
			;;
		*)
			printf 'x86_64\n'
			;;
	esac
}

fiber_target_rust_triple() {
	case "$(fiber_normalize_target "${1:-}")" in
		x86_64)
			printf 'x86_64-unknown-linux-gnu\n'
			;;
		aarch64)
			printf 'aarch64-unknown-linux-gnu\n'
			;;
	esac
}

fiber_target_debian_arch() {
	case "$(fiber_normalize_target "${1:-}")" in
		x86_64)
			printf 'amd64\n'
			;;
		aarch64)
			printf 'arm64\n'
			;;
	esac
}

fiber_target_kernel_meta_package() {
	case "$(fiber_normalize_target "${1:-}")" in
		x86_64)
			printf 'linux-image-amd64\n'
			;;
		aarch64)
			printf 'linux-image-arm64\n'
			;;
	esac
}

fiber_target_grub_platform() {
	case "$(fiber_normalize_target "${1:-}")" in
		x86_64)
			printf 'x86_64-efi\n'
			;;
		aarch64)
			printf 'arm64-efi\n'
			;;
	esac
}

fiber_target_efi_binary() {
	case "$(fiber_normalize_target "${1:-}")" in
		x86_64)
			printf 'BOOTX64.EFI\n'
			;;
		aarch64)
			printf 'BOOTAA64.EFI\n'
			;;
	esac
}

fiber_target_aliases() {
	case "$(fiber_normalize_target "${1:-}")" in
		x86_64)
			printf 'amd64\n'
			;;
		aarch64)
			printf 'arm64\n'
			;;
	esac
}

fiber_need_tool() {
	local tool="$1"
	command -v "$tool" >/dev/null 2>&1 || fiber_die "required tool '$tool' was not found"
}

fiber_copy_tree() {
	local source="$1"
	local target="$2"

	mkdir -p "$target"
	if command -v rsync >/dev/null 2>&1; then
		rsync -a --delete "$source"/ "$target"/
	else
		rm -rf "$target"
		mkdir -p "$target"
		cp -R "$source"/. "$target"/
	fi
}

fiber_overlay_tree() {
	local source="$1"
	local target="$2"

	[[ -d "$source" ]] || return 0
	mkdir -p "$target"
	if command -v rsync >/dev/null 2>&1; then
		rsync -a "$source"/ "$target"/
	else
		cp -R "$source"/. "$target"/
	fi
}

fiber_require_linux_for_iso() {
	case "$(uname -s)" in
		Linux)
			;;
		*)
			fiber_die "ISO assembly requires Linux tools. Run this on Linux or inside a Linux build VM/container"
			;;
	esac
}

fiber_sha256_cmd() {
	if command -v sha256sum >/dev/null 2>&1; then
		printf 'sha256sum\n'
	elif command -v shasum >/dev/null 2>&1; then
		printf 'shasum -a 256\n'
	else
		fiber_die "required tool 'sha256sum' or 'shasum' was not found"
	fi
}
