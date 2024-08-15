{
  description = "Simple daemon to publish Prometheus metrics from a cheap CO2 monitor";

  inputs.nixpkgs.url = "github:nixos/nixpkgs";

  outputs = { nixpkgs, ... }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
      manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
    in
    with pkgs;
    {
      devShells.${system}.default = mkShell {
        buildInputs = [
          rustc
          cargo
          clippy
          rust-analyzer
          rustfmt
        ];
        # Environment variables
        RUST_SRC_PATH = rustPlatform.rustLibSrc;
      };
      packages.${system}.default = rustPlatform.buildRustPackage {
        pname = manifest.name;
        version = manifest.version;
        src = lib.cleanSource ./.;
        cargoLock.lockFile = ./Cargo.lock;
        nativeBuildInputs = [ pkg-config ];
        buildInputs = [ udev ];
      };
    };
}
