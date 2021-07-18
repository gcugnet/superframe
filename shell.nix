{ pkgs ? import <nixpkgs> {} }:

with pkgs;

let
  gitflow = gitAndTools.gitflow;
in

mkShell {
  buildInputs = [
    # Build toolchain
    rustup

    # Other tools
    cargo-embed
    git
    gitflow
  ];
}
