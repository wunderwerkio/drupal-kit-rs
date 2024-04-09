{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-23.11";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
  }: flake-utils.lib.eachDefaultSystem (system:
    let
      overlays = [
        (import rust-overlay)
      ];
      pkgs = import nixpkgs {
        inherit system overlays;
      };
      rustToolchain = ./rust-toolchain.toml;
    in {
      devShells = rec {
        default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            pkg-config
            ((rust-bin.fromRustupToolchainFile rustToolchain).override { })
          ];

          buildInputs = with pkgs; [
            openssl
          ];
        };

      };

      formatter = pkgs.alejandra;
    }
  );
}
