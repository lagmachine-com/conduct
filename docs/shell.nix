with (import <nixpkgs> {});
let
  env = bundlerEnv {
    name = "docs-bundler-env";
    inherit ruby;
    gemfile  = ./Gemfile;
    lockfile = ./Gemfile.lock;
    gemset   = ./gemset.nix;
  };
in stdenv.mkDerivation {
  name = "docs";
  buildInputs = [ env ];
}
