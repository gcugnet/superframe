{ pkgs ? import <nixpkgs> {} }:

with pkgs;

mkShell {
  buildInputs = [
    # Add dependencies here.
    rustup
    cargo-embed
  ];
}
