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

        packages.default =
          let
            name = "Conny";
            version = "0.1.0";
            src = ./.;
          in
          pkgs.stdenv.mkDerivation {
            inherit name version src;

            cargoDeps = pkgs.rustPlatform.fetchCargoVendor {
              inherit src;
              hash = "sha256-mMfMA9t4rpJW4FsSRKrmxKo5QarOcI4KTakfFa2xuYw=";
            };

            nativeBuildInputs = [
              rustToolchain
              
              pkgs.pkg-config
              pkgs.meson
              pkgs.ninja
              pkgs.git
              pkgs.appstream
              pkgs.desktop-file-utils
              
              pkgs.rustPlatform.cargoSetupHook
              pkgs.wrapGAppsHook4
            ];

            buildInputs = [
              pkgs.gtk4
              pkgs.libadwaita
              pkgs.gettext
            ];

            meta = {
              description = "Use OpenVPN from GUI";
              homepage = "https://github.com/nikableh/Conny";
              license = pkgs.lib.licenses.mit;
            };
          };

        app.default = self.packages.${system}.default;

        devShells.default = pkgs.mkShell {
          inputsFrom = [ self.packages.${system}.default ];

          # Most of these dependencies are for building a Flatpak version of the
          # application.
          buildInputs = [
            pkgs.bashInteractive

            # https://github.com/NixOS/nixpkgs/issues/54312#issuecomment-455775414
            # Enable services.flatpak.enable = true; in configuration.nix, it
            # won't work without it.
            pkgs.flatpak
            pkgs.flatpak-builder
          ];
        };
      }
    );
}
