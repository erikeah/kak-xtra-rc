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
            name = "kakoune-extra-config";
            version = "0.1.0";
            src = ./.;
            cargoHash = "";
        };
        devShells.default = with pkgs; mkShell {
            packages = [ rust-analyzer rustfmt ];
            inputsFrom = [ config.packages.default ];
        };
      };
      flake = { };
    };
}
