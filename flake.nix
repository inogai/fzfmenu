{
  description = "A Nix-flake-based Rust development environment";

  nixConfig = {
    extra-substituters = [
      "https://cargo2nix.cachix.org"
    ];
    extra-trusted-public-keys = [
      "cargo2nix.cachix.org-1:ge7JNQYaRs+DO1o50vOUxRqLVze6G2VpPTt8EAg/b50="
    ];
  };

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = {self, ...} @ inputs: let
    supportedSystems = [
      "x86_64-linux"
      "aarch64-linux"
      "x86_64-darwin"
      "aarch64-darwin"
    ];
    forEachSupportedSystem = f:
      inputs.nixpkgs.lib.genAttrs supportedSystems (
        system:
          f {
            pkgs = import inputs.nixpkgs {
              inherit system;
            };
          }
      );
  in {
    devShells = forEachSupportedSystem (
      {pkgs}: {
        default = pkgs.mkShellNoCC {
          packages = with pkgs; [
            rustc
            cargo
            clippy
            rustfmt
            cargo-deny
            cargo-edit
            cargo-watch
            rust-analyzer
          ];

          env = {
            RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
          };
        };
      }
    );

    packages = forEachSupportedSystem (
      {pkgs}: {
        default = (pkgs.callPackage ./Cargo.nix {}).rootCrate.build;
      }
    );
  };
}
