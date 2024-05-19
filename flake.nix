{
  description = "Basic Rust flake for ryvex";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      with pkgs;
      {
        devShells.default = mkShell rec {
          buildInputs = [
            (rust-bin.selectLatestNightlyWith( toolchain: toolchain.default.override {
              extensions= [ "rust-src" "rust-analyzer" ];
              targets = [];
            }))

            pkg-config
            openssl
          ] ++ pkgs.lib.optionals pkg.stdenv.isDarwin [
            darwin.apple_sdk.frameworks.SystemConfiguration
          ];

          LD_LIBRARY_PATH = "${lib.makeLibraryPath buildInputs}";
        };
      }
    );
}
