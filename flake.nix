{
  description = "my project description";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, blender-bin, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system: let
        pkgs = import nixpkgs {
          system = system;
        };

        libraries = with pkgs; [
          rustc
          cargo
          rustfmt
          rust-analyzer
          clippy
          nodejs
          webkitgtk_4_1
          pkg-config
          libsoup_3
          gtk3
          webkitgtk_4_1
          cairo
          gdk-pixbuf
          libsoup_3
          gsettings-desktop-schemas
          glib
        ];

        in
        {
          devShell = pkgs.mkShell {
          buildInputs = libraries;
          LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath libraries}";
          SHELL = "${pkgs.bashInteractive}/bin/bash";
          WEBKIT_DISABLE_COMPOSITING_MODE=1;

          # see: https://github.com/tauri-apps/tauri/issues/7354
          shellHook = with pkgs; ''
              export XDG_DATA_DIRS=${gsettings-desktop-schemas}/share/gsettings-schemas/${gsettings-desktop-schemas.name}:${gtk3}/share/gsettings-schemas/${gtk3.name}:$XDG_DATA_DIRS;
            '';
          };
        }
    );
}
