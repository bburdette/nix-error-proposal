./debug-nix build --experimental-features nix-command --impure --show-trace --debugger --expr 'with import <nixpkgs> { }; callPackage ./attribute-missing.nix {}'
