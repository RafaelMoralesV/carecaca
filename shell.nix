{ pkgs ? import <nixpkgs> { } }:

with pkgs;

mkShell rec {
  shellHook = ''
  fish
  '';
  nativeBuildInputs = [
    pkg-config
  ];
  buildInputs = [
    udev alsa-lib vulkan-loader
    xorg.libX11 xorg.libXcursor xorg.libXi xorg.libXrandr # To use the x11 feature
    libxkbcommon wayland # To use the wayland feature
    mold
  ];
  LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
  NIX_ENFORCE_PURITY= 0;
  CARGO_TARGET_DIR = "target/project";
}
