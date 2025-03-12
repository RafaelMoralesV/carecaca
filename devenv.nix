{ pkgs, lib, config, inputs, ... }:

{
  env = {
    LD_LIBRARY_PATH = lib.makeLibraryPath config.packages;
    NIX_ENFORCE_PURITY = 0;
    CARGO_TARGET_DIR = "target/project";
  };

  # https://devenv.sh/packages/
  packages = with pkgs; [
    git
    cargo-watch
    udev
    alsa-lib
    vulkan-loader
    mold

    # To use the x11 feature
    xorg.libX11
    xorg.libXcursor
    xorg.libXi
    xorg.libXrandr

    # To use the wayland feature
    libxkbcommon
    wayland
  ];

  # https://devenv.sh/languages/
  languages.rust.enable = true;

  # https://devenv.sh/processes/
  processes.cargo-watch.exec = "cargo-watch";

  # https://devenv.sh/services/
  # services.postgres.enable = true;

  # https://devenv.sh/tasks/
  # tasks = {
  #   "myproj:setup".exec = "mytool build";
  #   "devenv:enterShell".after = [ "myproj:setup" ];
  # };

  # https://devenv.sh/tests/
  enterTest = ''
    echo "Running tests"
    cargo test -p clown
    cargo test -p circus
    cargo test -p shitface
  '';

  # https://devenv.sh/git-hooks/
  # git-hooks.hooks.shellcheck.enable = true;

  # See full reference at https://devenv.sh/reference/options/
}
