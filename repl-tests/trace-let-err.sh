./debug-nix build --experimental-features nix-command --impure --debugger  --show-trace --expr 'with import <nixpkgs> { }; callPackage ./let-err.nix {}'
