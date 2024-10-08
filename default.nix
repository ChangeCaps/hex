{ pkgs ? import <nixpkgs> {} }:

pkgs.stdenv.mkDerivation rec {
  name = "ori";

  buildInputs = [
    pkgs.libGL

    pkgs.wayland
    pkgs.libxkbcommon
    pkgs.xorg.libX11
    pkgs.pkg-config
  ];

  LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath buildInputs}";
}
