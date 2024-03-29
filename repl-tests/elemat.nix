{ stdenv
, fetchFromGitHub
, rustPlatform
, openssl
, pkgconfig
, sqlite
, lib
, callPackage }:

# , lib
# , packr

rustPlatform.buildRustPackage rec {
  pname = "zknotes-server";
  version = "1.0";

  test = ["meh" "you" "bburdette"];

  idx = (break 5);

  # ui = callPackage ./ui.nix { };

  src = fetchFromGitHub {
    # thunk error.
    # owner = builtins.elemAt test (break idx);
    # works
    owner = builtins.elemAt test idx;
    repo = "zknotes";
    rev = "8166ed086f058e537bdbe8971192b1085acecc30";
    sha256 = "13fbzn5m46jm53lxq2ccby427pgb8a08v8giz4xl560d19xjjb6f";
  };

  # preBuild = ''
  #   cp -r ${ui}/libexec/gotify-ui/deps/gotify-ui/build ui/build && packr
  # '';

  # postInstall = ''
  #   echo "postInttall"
  #   ls -l $out
  #   cp -r ${ui}/static $out
  # '';

  # cargo-culting this from the gotify package.
  subPackages = [ "." ];


  sourceRoot = "source/server";
  cargoSha256 = "1sx9ihzwn1vb1dx48ykzg1b9xdbjk4cbzs89labhgljhh51sr64p";
  # dontMakeSourcesWritable=1;

  buildInputs = [openssl sqlite];

  nativeBuildInputs = [ pkgconfig ];

  meta = with lib; {
    description = "zknotes zettelkasten server.";
    homepage = https://github.com/bburdette/zknotes;
    license = licenses.gpl3;
    maintainers = [ ];
    platforms = platforms.all;
  };
}

