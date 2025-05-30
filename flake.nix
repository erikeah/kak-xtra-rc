{
  description = "kak-xtra-rc: unclutter your kakrc";

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [  ];
      systems = [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" "x86_64-darwin" ];
      perSystem = { config, self', inputs', pkgs, system, ... }: {
        packages.default = pkgs.rustPlatform.buildRustPackage {
            name = "kak-xtra-rc";
            version = "0.1.0";
            src = ./.;
            cargoHash = "sha256-3Zuc7v/jHqY2bOc2vUeqruVprM2dFYLagmr/2e9EBbs=";
        };
        devShells.default = with pkgs; mkShell {
            packages = [ rust-analyzer rustfmt ];
            inputsFrom = [ config.packages.default ];
        };
      };
      flake = { };
    };
}
