{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    { self
    , nixpkgs
    , flake-utils
    , ...
    }:
    flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = import nixpkgs {
        inherit system;
      };

      anyrun-shell-shortcuts = pkgs.rustPlatform.buildRustPackage {
        pname = "anyrun-shell-shortcuts";
        version = "0.1.0";
        cargoLock.lockFile = ./Cargo.lock;

        src = pkgs.lib.cleanSource ./.;
      };
    in
    {
      packages = {
        anyrun-shell-shortcuts = anyrun-shell-shortcuts;
        default = anyrun-shell-shortcuts;
      };
    }
    );
}
