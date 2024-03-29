{ stdenv
, fetchFromGitHub
, rustPlatform
, openssl
, pkgconfig
, sqlite
, lib
, callPackage }:

rustPlatform.buildRustPackage rec {
  pname = "zknotes-server";
  version = "1.0";

  # ui = callPackage ./ui.nix { };

  src = fetchFromGitHub {
    owner = "bburdette";
    repo = "zknotes";
    rev = "8166ed086f058e537bdbe8971192b1085acecc30";
    sha256 = "13fbzn5m46jm53lxq2ccby427pgb8a08v8giz4xl560d19xjjb6f";
  };

  subPackages = [ "." ];


  sourceRoot = "source/server";
  cargoSha256 = "1sx9ihzwn1vb1dx48ykzg1b9xdbjk4cbzs89labhgljhh51sr64p";
  # dontMakeSourcesWritable=1;

  buildInputs = [openssl sqlite];

  nativeBuildInputs = [ pkgconfig ];

  meta = with lib; {
    description = "zknotes zettelkasten server.";
    homepage = https://github.com/bburdette/zknotes;
    license = [ gplX ];
    maintainers = [ ];
    platforms = platforms.all;
  };
}

