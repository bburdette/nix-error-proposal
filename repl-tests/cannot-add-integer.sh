./debug-nix build  --experimental-features nix-command --impure --debugger --expr 'with import <nixpkgs> { }; callPackage ./cannot-add-integer.nix {}'
