{
  mozillaOverlay ? import (
    builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz
  )
}:
let
  nixpkgs = import <nixpkgs> { overlays = [ mozillaOverlay ]; };
  rustChannel = nixpkgs.latest.rustChannels.stable.rust.override {
    extensions = [
      "rust-src"
      "rls-preview"
      "clippy-preview"
      "rustfmt-preview"
    ];
  };
in
with nixpkgs; mkShell {
  buildInputs = [cargo rustChannel rustc rustfmt rustup rls];
}
