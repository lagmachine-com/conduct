
let pkgs = import <nixpkgs>{};

shell = pkgs.mkShell {
  buildInputs = with pkgs; [
    inkscape
  ];

  WEBKIT_DISABLE_COMPOSITING_MODE=1;
};

in shell
