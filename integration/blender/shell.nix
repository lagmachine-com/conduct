
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
};

in shell
