{
	description = "FiberOS";

	inputs = {
		nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
		nixos-generators.url = {
			url = "github:nix-community/nixos-generators";
			inputs.nixpkgs.follows = "nixpkgs";
		};

		home-manager = {
			url = "github:nix-community/home-manager";
			inputs.nixpkgs.follows = "nixpkgs";
		};
	};

	outputs = { self, nixpkgs, nixos-generators, home-manager, ... };

	let supportedSystems = ["x86_64-linux", "aarch64-linux", "riscv64-linux"];

	in {
		nixosConfigurations.fiberos-x86_64 = mkSystem { system = "x86_64-linux"; kernelSel = zen; };
		nixosConfigurations.fiberos-aarch64 = mkSystem { system = "aarch64-linux"; kernelSel = lts; };
		nixosConfigurations.fiberos-riscv64 = mkSystem { system = "riscv64-linux"; kernelSel = lts; };
	}

	mkInstaller = { system }: nixos-generators.nixosGenerate {
		inherit system;

	};
}
