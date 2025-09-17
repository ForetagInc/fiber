{ config, pkgs, lib, ... }:

{
	networking.hostName = "fiberos";
	time.timeZone = "UTC";

	#
	# Media [Audio & Video]
	#
	security.rtkit.enable = true;
	services.pipewire = {
		enable = true;
		alsa.enable = true;
		pulse.enable = true;
		jack.enable = false;
		wireplumber.enable = true;
	}

	# USB & Thunderbolt
	services.hardware.bolt.enable = true;
	systemd.services.bolt.enable = true;
	services.upower.enable = true;
	power-profiles-daemon.enable = true;
	hardware.usb-modeswitch.enable = true;

	# Bluetooth
	hardware.bluetooth.enable = true;
	hardware.bluetooth.powerOnBoot = true;

	#
	# Network
	#
	networking.networkmanager.enable = true;

	# Packages
	environment.systemPackages = with pkgs; [
		chromium
		monado
		openxr-loader
	];

	boot.kernel.sysctl = {
		"kernel.kptr_restrict" = 2;
		"kernel.unprivileged_bpf_disabled" = 1;
	};

 	documentation.enable = false;
	documentation.man.enable = false;
	programs.command-not-found.enable = false;
	nix.settings.auto-optimise-store = true;
}
