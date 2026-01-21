{
  description = "Engram - Distributed Memory System for AI Agents (Rust Implementation)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    crane.url = "github:ipetkov/crane";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, crane }:
    let
      pkgs = import nixpkgs {
        system = "x86_64-linux";
        overlays = [ rust-overlay.overlays.default ];
      };
      
      rustToolchain = pkgs.rust-bin.stable.latest.default;
      
      craneLib = crane.mkLib pkgs;
      
      engramPackage = craneLib.buildPackage {
        pname = "engram";
        version = "0.1.0";
        src = craneLib.cleanCargoSource ./.;
        
        cargoLockFile = ./Cargo.lock;

        doCheck = false;

        OPENSSL_NO_VENDOR = "1";
        OPENSSL_DIR = "${pkgs.openssl.dev}";
        OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
        OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include";
        PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
      };
    in
    {
      packages.x86_64-linux.default = engramPackage;
      packages.x86_64-linux.engram = engramPackage;
      
      devShells.x86_64-linux.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          rustToolchain
          pkg-config
          openssl
          openssl.dev
          git
          rust-analyzer
          perl
        ];

        shellHook = ''
          export PKG_CONFIG_PATH="${pkgs.openssl.dev}/lib/pkgconfig:$PKG_CONFIG_PATH"
          export OPENSSL_DIR="${pkgs.openssl.dev}"
          export OPENSSL_LIB_DIR="${pkgs.openssl.out}/lib"
          export OPENSSL_INCLUDE_DIR="${pkgs.openssl.dev}/include"
          export OPENSSL_NO_VENDOR="1"
          echo 'Engram Rust development environment ready with OpenSSL support'
        '';
      };

      checks.x86_64-linux.default = pkgs.rustPlatform.buildRustPackage {
        pname = "engram-check";
        version = "0.1.0";
        src = ./.;
        
        cargoLock = {
          lockFile = ./Cargo.lock;
        };

        nativeBuildInputs = with pkgs; [ 
          pkg-config 
          rustToolchain 
          perl 
        ];
        
        buildInputs = with pkgs; [ 
          openssl 
          openssl.dev 
          git 
        ];

        OPENSSL_NO_VENDOR = "1";
        OPENSSL_DIR = "${pkgs.openssl.dev}";
        OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
        OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include";

        doCheck = true;
        checkPhase = ''
          cargo test --workspace
          cargo clippy -- -D warnings
        '';
        
        installPhase = ''
          echo "Checks completed"
          mkdir -p $out
          touch $out/check-results
        '';
      };
    };
}
