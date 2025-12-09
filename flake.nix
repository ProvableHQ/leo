{
  description = "The Leo programming language";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        
        # Read Rust toolchain from rust-toolchain.toml
        rust = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        
        # Read version from Cargo.toml
        cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
        version = cargoToml.package.version;
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rust
            curl
            openssl
            pkg-config
            # Additional tools that might be useful
            cargo-watch
            cargo-udeps
            rust-analyzer
          ];

          shellHook = ''
            echo "Leo development environment"
            echo "Rust version: $(rustc --version)"
            echo "Cargo version: $(cargo --version)"
          '';

          # Set environment variables for Rust
          RUST_BACKTRACE = "1";
          
          # Ensure pkg-config can find openssl
          PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
        };

        # Optional: provide a package for building leo
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "leo-lang";
          inherit version;
          src = ./.;
          
          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          nativeBuildInputs = with pkgs; [
            pkg-config
            openssl
          ];

          buildInputs = with pkgs; [
            openssl
          ];

          # Build flags
          buildFeatures = [ ];
          
          # Skip tests during build (can be overridden)
          doCheck = false;
        };

        # App to run leo via nix run
        apps.default = {
          type = "app";
          program = "${self.packages.${system}.default}/bin/leo";
        };
      }
    );
}
