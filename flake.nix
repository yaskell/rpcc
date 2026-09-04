{
  description = "Devshell for Crust";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = {
    self,
    nixpkgs,
  }: let
    system = "x86_64-linux";
    pkgs = import nixpkgs {inherit system;};
  in {
    devShells.${system}.default = pkgs.mkShell {
      buildInputs = with pkgs; [
        gcc
        gdb
        python315 # Used for running test suite
        cargo
        rustc
        rustfmt
        clippy
        bacon
        rust-analyzer
      ];
      shellHook = ''
        export RUST_SRC_PATH=${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}
        export PATH=$PATH:$(pwd)/target/release
        rustc --version
        echo
      '';
    };
  };
}
