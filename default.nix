let
  _nixpkgs = import <nixpkgs> {};
  fetchNixPkgs = { rev, sha256 }:
    _nixpkgs.fetchFromGitHub {
      inherit sha256 rev;
      owner = "NixOS";
      repo = "nixpkgs-channels";
    };
    pkgs = import (fetchNixPkgs {
      rev = "39cd40f7bea40116ecb756d46a687bfd0d2e550e";
      sha256 = "0kpx4h9p1lhjbn1gsil111swa62hmjs9g93xmsavfiki910s73sh";
    }) {};

in with pkgs;
stdenv.mkDerivation rec {
  name = "visu-${version}";
  version = "0.1";

  buildInputs = [
    rust-env
  ];

  depsLibs = with xorg; [
    expat
    freetype
    fontconfig
    libX11
    libXcursor
    libXxf86vm
    libXi
    libXrandr
    mesa
    ];

  LD_LIBRARY_PATH="${lib.makeLibraryPath depsLibs}";
}
