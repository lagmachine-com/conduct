
let unstableTarball = fetchTarball https://github.com/NixOS/nixpkgs/archive/nixos-unstable.tar.gz;
pkgs = import <nixpkgs> {};
unstable = import unstableTarball {};

shell = pkgs.mkShell {
  buildInputs = with pkgs; [
    unstable.rustc
    unstable.cargo
    unstable.rustfmt
    unstable.rust-analyzer
    unstable.clippy
    nodejs
    webkitgtk_4_1
    pkg-config
    libsoup_3
  ];

  RUST_SRC_PATH = "${unstable.rust.packages.stable.rustPlatform.rustLibSrc}";
  RUST_BACKTRACE = 1;
  WEBKIT_DISABLE_COMPOSITING_MODE=1;
};

in shell
