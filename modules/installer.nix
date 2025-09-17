{ config, pkgs, lib, ... }:

{
	isoImage.squashfsCompression = "zstd -Xcompression-level 15";

	services.xserver.enable = true;
	services.xserver.desktopManager.plasma6.enable = true;
	services.displayManager.sddm.enable = true;

	networking.networkmanager.enable = true;
	services.pipewire = {
		enable = true;
		alsa.enable = true;
		pulse.enable = true;
	};

	services.calamares = {
		enable = true;
		settings = lib.importTOML ./../calamares/settings.conf;
		modules = [ ];
		branding = {
			name = "fiberos";
			directory = ./../calamares/branding/fiberos;
		};
	};

	environment.etc."calamares/settings.conf".source = ./../calamares/settings.conf;

	 boot.plymouth.enable = true;

	users.users.nixos = {
		isNormalUser = true;
		extraGroups  = [ "wheel" "networkmanager" "video" "audio" "input" ];
		initialPassword = "fiber";
	};

	security.sudo.wheelNeedsPassword = false;

	documentation.enable = false;
	documentation.man.enable = false;
	programs.command-not-found.enable = false;
	nix.settings.auto-optimise-store = true;
}
