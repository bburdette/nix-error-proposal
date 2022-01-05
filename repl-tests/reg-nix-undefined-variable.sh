# ./debug-nix build  --experimental-features nix-command --impure --debugger --expr 'with import <nixpkgs> { }; callPackage ./undefined-variable.nix {}'
nix build  --experimental-features nix-command --impure --show-trace --expr 'with import <nixpkgs> { }; callPackage ./undefined-variable.nix {}'
