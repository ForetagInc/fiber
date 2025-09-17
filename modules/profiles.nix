{ lib, ... }:

with lib; {
	options.fiber.profile = mkOption {
		type = types.enum [ "dev", "standard", "privacy" ];
		default = "standard";
		description = "FiberOS Profile";
	};

	options.fiber.xr.enableByDefault = mkOption {
		type = types.bool;
		default = false;
		description = "Autostart XR session";
	};
}
