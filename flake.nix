{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-24.11";
    utils.url = "github:wunderwerkio/nix-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    self,
    nixpkgs,
    utils,
    rust-overlay,
  }: utils.lib.systems.eachDefault (system:
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
