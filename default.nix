with import <nixpkgs> {};

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
    ];

  LD_LIBRARY_PATH=(builtins.getEnv "LD_LIBRARY_PATH") + "${lib.makeLibraryPath depsLibs}";
}
