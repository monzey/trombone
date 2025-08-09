{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };
  };

 outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ rust-overlay.overlays.default ];
        pkgs = import nixpkgs {
          inherit system;
          inherit overlays;
        };
        
        rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./api/rust-toolchain.toml;
      in
      {
        devShells.default = pkgs.mkShell {
          packages = [
            rustToolchain
            pkgs.rust-analyzer
            pkgs.postgresql
            pkgs.dotenv-cli
            pkgs.sqlx-cli
            pkgs.pkg-config
            pkgs.openssl
            pkgs.cargo-watch
          ];
        };
      }
    );
}
