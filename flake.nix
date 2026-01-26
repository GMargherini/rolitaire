{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        devShell = with pkgs; mkShell {
          buildInputs = [
            cargo
            rustc
          ];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
        };
        devShells.${system}.default = pkgs.mkShell
        {
          packages = with pkgs; [
            rust-analyzer
            rustfmt
            pre-commit
            rustPackages.clippy
          ];
        };
      }
    );
}
