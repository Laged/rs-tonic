let
  moz_overlay = import (builtins.fetchTarball
    "https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz");
  nixpkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
in with nixpkgs;
stdenv.mkDerivation {
  name = "moz_overlay_shell";
  buildInputs =
    [ latest.rustChannels.beta.rust openssl pkg-config protobuf  ];
  PROTOC = "${pkgs.protobuf}/bin/protoc";
}
