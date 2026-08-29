{
  description = "Stick driver development flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs =
    { self, nixpkgs, ... }:
    let
      systems = [
        "aarch64-linux"
        "x86_64-linux"
        "aarch64-darwin"
        "x86_64-darwin"
      ];
      forEachSystem = fn: nixpkgs.lib.genAttrs systems (system: fn nixpkgs.legacyPackages.${system});
    in
    {
      devShells = forEachSystem (pkgs: {
        default = pkgs.mkShell {
          packages =
            with pkgs;
            [
              cargo
              rustc
              clippy
              rustfmt
              rust-analyzer
            ]
            # gilrs-core's Linux backend links against libudev at build
            # time (via pkg-config); its macOS backend uses IOKit/
            # CoreFoundation frameworks instead, which don't need anything
            # from Nix, so this is Linux-only.
            ++ lib.optionals stdenv.hostPlatform.isLinux [
              pkg-config
              udev
            ];
          RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
        };
      });

      packages = forEachSystem (pkgs: {
        default = pkgs.rustPlatform.buildRustPackage {
          pname = "stick_shift";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
          nativeBuildInputs = pkgs.lib.optionals pkgs.stdenv.hostPlatform.isLinux [ pkgs.pkg-config ];
          buildInputs = pkgs.lib.optionals pkgs.stdenv.hostPlatform.isLinux [ pkgs.udev ];
        };
      });
    };
}
