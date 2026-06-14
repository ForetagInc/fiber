# Fiber

A lightweight operating system that is open, secure, performant, and extensible. Fiber targets x86_64 and AArch64 first.

- Free & Open source, always will be
- Web based: Open marketplace via Web Apps
- Multi-surface compositor: Desktop, mobile, and OpenXR-backed spatial shells
- Lightweight: Designed for minimal resource usage
- Secure: Disk Encryption and Secure Boot
- Anonymous Mode with Tor

## Architecture

Fiber Compositor is designed as a shared compositor core with multiple shell frontends:

- Fiber Desktop Shell: desktop/laptop UI and the first release target.
- Fiber Mobile Shell: touch-first phone, tablet, and handheld UI.
- Fiber Spatial Shell: OpenXR-backed spatial UI for 2D panels and immersive applications.

The core models Wayland surfaces, app focus, input routing, output management, permissions, screen capture mediation, frame scheduling, lifecycle hooks, accessibility hooks, and XWayland compatibility where needed. Shells share the same app platform, design tokens, lifecycle model, permission prompts, notifications, and update system.

See the architecture contracts in `crates/fiber_compositor` and the accepted decisions in `docs/adr`.

## Requirements

### Minimum

- Memory: 2 GB (+ zram)
- CPU: Dual Core
- GPU: iGPU (Vulkan Support)
- Storage: 20GB

> XR requires a quad core CPU, 8 GB RAM (+ zram), iGPU / GTX 1660 or newer, an OpenXR runtime such as Monado, and 128 GB Storage.

## Recommended

- Memory: 16 GB
- CPU: 6 Core / 16 Threads
- GPU: RX 6600 / Arc A580 / RTX 2060
- Storage: 256 GB NVMe

> For Development and Gaming Profiles, you may need additional storage

## Building

Fiber now uses a custom image pipeline instead of a NixOS-based build.

The build pipeline now has one release entrypoint that can either:

- consume an existing rootfs, kernel, and initrd per target, or
- bootstrap those Linux inputs automatically on a Linux build host/CI runner.

During rootfs staging, the pipeline builds or installs Fiber's compositor binary,
adds the `fiber-compositor.service` systemd unit, and sets the default graphical
boot path to start the spatial Wayland/OpenXR compositor service.

For cross builds, pass a prebuilt compositor with `--compositor-bin` or install
the matching Rust target and linker on the Linux build host.

Hardware support is verified during rootfs staging with architecture-aware
checks. By default missing pieces are reported as warnings; production builds
should use `--strict-hardware`.

Wall-clock synchronization is owned by `systemd-timesyncd` in the OS image.
Fiber configures `time.cloudflare.com` as the NTP source for system clock sync.

Tor support is built into the OS image. The installer persists the selected
Fiber profile, and `Hacker` mode enables `tor.service` automatically on the
installed system so it joins the Tor network on boot. Other profiles keep Tor
installed but disabled by default.
Until Fiber ships a profile selection UI, the Calamares integration reads the
profile from `FIBER_PROFILE` and defaults to `User`.

Build both release ISOs on a Linux build host:

```bash
scripts/build-release --bootstrap-rootfs --with-aliases
```

Build a single target from existing Linux artifacts:

```bash
scripts/build-release \
  --target x86_64 \
  --rootfs-source /srv/fiber/{target}/rootfs \
  --kernel /srv/fiber/{target}/vmlinuz \
  --initrd /srv/fiber/{target}/initrd.img \
  --strict-hardware
```

Useful lower-level steps:

- `scripts/bootstrap-rootfs --target aarch64`
- `scripts/build-rootfs --source /srv/fiber/rootfs --target x86_64`
- `scripts/build-compositor --target x86_64`
- `scripts/verify-rootfs-hardware --rootfs dist/rootfs/x86_64 --strict`
- `scripts/build-kernel --kernel /srv/fiber/kernel/vmlinuz --initrd /srv/fiber/kernel/initrd.img`
- `scripts/build-image --target x86_64`

`scripts/build-release --with-aliases` also emits compatibility ISO filenames
for `amd64` and `arm64` beside the canonical `x86_64` and `aarch64` outputs.

GitHub Actions now builds the release ISOs for version tags matching `v*`
using the same `scripts/build-release` entrypoint. The workflow uses:

- `ubuntu-24.04` for `x86_64`
- `ubuntu-24.04-arm` for `aarch64`

Required Linux build tools are listed in `image/build-tools.txt`.
