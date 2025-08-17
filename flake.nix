{
  description = "Nix flake of the project";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-25.05";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      fenix,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        rustToolchain =
          (fenix.packages.${system}.fromToolchainName {
            name = (pkgs.lib.importTOML ./rust-toolchain.toml).toolchain.channel;
            sha256 = "sha256-+9FmLhAOezBZCOziO0Qct1NOrfpjNsXxc/8I0c7BdKE=";
          }).toolchain;
      in
      {
        formatter = pkgs.nixfmt-rfc-style;

        devShells.default = pkgs.mkShell {
          buildInputs = [
            rustToolchain
            pkgs.bashInteractive
            pkgs.git
            pkgs.coreutils
            pkgs.gtk4
            pkgs.pkg-config
            pkgs.libadwaita
            pkgs.meson
          ];
        };
      }
    );
}
