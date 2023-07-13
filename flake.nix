{
  description = "Rust devshell for developing my website with Leptos";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
    
    flake-utils.url = "github:numtide/flake-utils";
    
    cargo-leptos = {
      url = "github:benwis/cargo-leptos"; #Has fixes that are not in original for this to build
      flake = false;
    };

  };
  
  outputs = { self, nixpkgs, rust-overlay, flake-utils, cargo-leptos,...}: 
    flake-utils.lib.eachDefaultSystem (system: 
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        leptos = pkgs.rustPlatform.buildRustPackage rec {
          pname = "cargo-leptos";
          version = "0.1.8.1";
          buildFeatures = ["no_downloads"];

          src = cargo-leptos;

          cargoSha256 = "sha256-+AjgFKhcOMY2yi49+f85NukG3NBkNFDx3OrNQxvs31o=";

          nativeBuildInputs = [pkgs.pkg-config pkgs.openssl];

          buildInputs = with pkgs; [
              openssl
              pkg-config
          ];

          doCheck = false;

          meta = with pkgs.lib; {
            description = "A build tool for the leptos web framework";
            homepage = "https://github.com/leptos-rs/cargo-leptos";
            changelog = "https://github.com/leptos-rs/cargo-leptos/blob/v${version}/CHANGELOG.md";
            license = with licenses; [mit];
            maintainers = with maintainers; [benwis];
          };
        };
      in
      with pkgs;
      {
        devShells.default = mkShell {
          buildInputs = [
            (rust-bin.selectLatestNightlyWith(toolchain: toolchain.default.override {
              extensions= [ "rust-src" "rust-analyzer" ];
              targets = [ "wasm32-unknown-unknown" ];
            }))
            leptos
            sass
            binaryen
            wasm-pack
            nodePackages.tailwindcss
            pkg-config
            jq
            cachix
          ];
        };
      }
    );
}