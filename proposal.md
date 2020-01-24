
The 3 type of nix errors.

# class 1:  nix language errors.

- most like elm errors.
- use error template as below.
- error message catalog.
  collection of *.nix snippets that produce errors.

Language error template:

      ---<Error Type>----------------------------- <filename>

      general error text.

      <line number> line of code where error occurred, with color underlining.
                         ^^ 

      hint about how to fix the problem, perhaps with text indicating the problem:
        which occurred at this word: <of>

# class 2:  tool error messages.

- line/column not appropriate, as much.

- much more varied requirements for producing the messages.
  - detecting problem conditions. 

error template:

--<Error type>------------------------------- <tool name>

  general error text.
  
  tool command line args.

  proposed solution?

  docs link.

# class 3:  bash error messages.

it would be great to have line-by-line annotation for the tool.
  but, at least we could have the calling nix file and build phase.

here's an example: 

    bburdette@BB-5520:~/code/pdfq/elm$ nix-build parcel2.nix 
    these derivations will be built:
      /nix/store/m2p6lahccxnkvxv7yiwz37i8blqgdbls-myproject-node-packages.drv
      /nix/store/ilhkpii2f8ai900vnzcmzb53an0dqz8a-myproject-frontend.drv
    building '/nix/store/m2p6lahccxnkvxv7yiwz37i8blqgdbls-myproject-node-packages.drv'...
    unpacking sources
    patching sources
    configuring
    mv: cannot copy a directory, '/build', into itself, '/build/temp'
    builder for '/nix/store/m2p6lahccxnkvxv7yiwz37i8blqgdbls-myproject-node-packages.drv' failed with exit code 1
    cannot build derivation '/nix/store/ilhkpii2f8ai900vnzcmzb53an0dqz8a-myproject-frontend.drv': 1 dependencies couldn't be built
    error: build of '/nix/store/ilhkpii2f8ai900vnzcmzb53an0dqz8a-myproject-frontend.drv' failed

where is 'mv' being called?  from what nix file? in what stage?
  ok it was this myproject-node-packages

error context stack.
  in bash, we get:
    chmod: changing permissions of '/nix/store/l8dyvv1s85sfpw672f64l9ynx3adx9mh-source/server/': Operation not permitted
  how about have chmod get called by something that will print both the chmod error and the context around it.
    - why is chmod being called?
    - if you get this error, what should you do?
    - where should you look in the docs to get help?
    bburdette@BB-5520:~/code/pdfq/nixpkg$ sudo ./test.sh 
      [sudo] password for bburdette: 
      these derivations will be built:
        /nix/store/pvy84fi7sp1l672s4iijkq4srrism7ic-pdfq-1.0-vendor.drv
        /nix/store/gal5wxp40cwsayghisxajv3l70qv9n5w-pdfq-1.0.drv
      building '/nix/store/pvy84fi7sp1l672s4iijkq4srrism7ic-pdfq-1.0-vendor.drv'...
      unpacking sources
      unpacking source archive /nix/store/l8dyvv1s85sfpw672f64l9ynx3adx9mh-source
      source root is /nix/store/l8dyvv1s85sfpw672f64l9ynx3adx9mh-source/server/
      chmod: changing permissions of '/nix/store/l8dyvv1s85sfpw672f64l9ynx3adx9mh-source/server/': Operation not permitted
      chmod: changing permissions of '/nix/store/l8dyvv1s85sfpw672f64l9ynx3adx9mh-source/server/shell.nix': Operation not permitted
      chmod: changing permissions of '/nix/store/l8dyvv1s85sfpw672f64l9ynx3adx9mh-source/server/config.toml': Operation not permitted
      chmod: changing permissions of '/nix/store/l8dyvv1s85sfpw672f64l9ynx3adx9mh-source/server/run.sh': Operation not permitted


  
