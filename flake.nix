{
  description = "Devshell for Crust";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = {nixpkgs, ...}: let
    system = "x86_64-linux";
    pkgs = import nixpkgs {inherit system;};
  in {
    devShells.${system}.default = pkgs.mkShell {
      packages = with pkgs; [
        bacon
        cargo
        clippy
        gcc
        gdb
        python315 # Used for running test suite
        rust-analyzer
        rustc
        rustfmt
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
