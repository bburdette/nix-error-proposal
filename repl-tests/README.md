These are some test scripts I've been using to work on the nix --debugger flag.

To test, check out the debugger branch from the PR:

https://github.com/NixOS/nix/pull/5416

Build nix in the usual way.

Then clone this repo, and make a symbolic link to point at your executable.  For me its

`ln -s ../../nix/outputs/out/bin/nix debug-nix`

Then you can try out some of these scripts.  Not all put you in the debuggger,
but these do:

./attribute-missing.sh
./cannot-add-float.nix
./cannot-add-integer.nix
./attribute-missing.sh
./nix-build.sh
./missing-argument.sh
./undefined-variable.sh
./trace-let-err.sh


