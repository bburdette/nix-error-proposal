nix-build -E --debugger 'with import <nixpkgs> { }; callPackage ./default.nix {}'
