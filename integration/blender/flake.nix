{
  description = "Blender development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    blender-bin.url = "github:edolstra/nix-warez?dir=blender";
    nixgl.url = "github:guibou/nixGL";
  };

  outputs = { self, blender-bin, nixpkgs, nixgl, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          system = system;
          overlays = [nixgl.overlay];
        };

        blender-binPkgs = blender-bin.packages.${system};

        packages = [
          blender-binPkgs.blender_4_0
        ];

        libraries = with pkgs; [
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
          buildInputs = packages ++ libraries;
          LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath libraries}";
          SHELL = "${pkgs.bashInteractive}/bin/bash";
          WEBKIT_DISABLE_COMPOSITING_MODE = 1;
          BLENDER_USER_EXTENSIONS = "~/Documents/Blender/scripts";

          # see: https://github.com/tauri-apps/tauri/issues/7354
          # Requires nixGL to be installed to the system!
          shellHook = with pkgs; ''
            export XDG_DATA_DIRS=${gsettings-desktop-schemas}/share/gsettings-schemas/${gsettings-desktop-schemas.name}:${gtk3}/share/gsettings-schemas/${gtk3.name}:$XDG_DATA_DIRS;
            alias blender="nixGL ${blender-binPkgs.blender_4_0}/bin/blender";
          '';
        };
      }
    );
}