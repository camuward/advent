{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs: inputs.flake-parts.lib.mkFlake { inherit inputs; } {
    systems = [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];
    perSystem = { pkgs, ... }: {
      devShells.default = pkgs.mkShell {
        packages = [ pkgs.bacon ];

        buildInputs = [
          pkgs.openssl
        ];

        nativeBuildInputs = let
          rust-bin = inputs.rust-overlay.lib.mkRustBin {} pkgs;
        in [
          pkgs.pkg-config
          (rust-bin.fromRustupToolchainFile ./rust-toolchain.toml)
          (pkgs.lib.optionals pkgs.stdenv.isLinux pkgs.mold-wrapped)
        ];

        env = {
          # Required by rust-analyzer
          # RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
        };

        shellHook = ''
          rustc -Vv
        '' + pkgs.lib.optionalString pkgs.stdenv.isLinux ''
          export RUSTFLAGS="-Clink-arg=-fuse-ld=mold $RUSTFLAGS"
        '';
      };
    };

    flake = {};
  };
}
