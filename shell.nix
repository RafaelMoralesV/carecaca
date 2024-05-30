{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
    nativeBuildInputs = with pkgs; [
        gcc
        gnumake
        cargo
        rustc
        rustfmt
        clippy
        bacon
    ];
}
