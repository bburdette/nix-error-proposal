# ./debug-nix build  --experimental-features nix-command --impure --debugger --expr 'with import <nixpkgs> { }; callPackage ./undefined-variable.nix {}'
./debug-nix build  --experimental-features nix-command --impure --show-trace --debugger --expr 'with import <nixpkgs> { }; callPackage ./undefined-variable.nix {}'
