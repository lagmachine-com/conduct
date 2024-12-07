
let pkgs = import <nixpkgs> {
  config = {
    allowUnfree = true;
  };
};

shell = pkgs.mkShell {
  buildInputs = with pkgs; [
    (blender.override {
    cudaSupport = true;
    })
  ];

  WEBKIT_DISABLE_COMPOSITING_MODE=1;
};

in shell
