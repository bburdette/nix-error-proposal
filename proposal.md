# overview

We want to improve nix errors - the messages themselves, their context, and formatting.

I've gone through the nix github issues, and come up with about 90 candidates for improved errors.
Its likely there are many more improvements to be made, but these are some representative examples 
that point at the kinds of changes that would be appropriate.

Here's a spreadsheet with the candidate errors at the top:
https://docs.google.com/spreadsheets/d/1YeMT8nQPaMaZWLKE0IqVY5o8XvfiNbhuv1TWWZ0VwJk/edit#gid=1201267462

I've put these into the following four classes:
  * *language*: syntax errors and the like in the nix language. (13 issues)
  * *builtin*: errors that occur in builtin functions. (6 issues)
  * *tool*: errors returned by tools like nix-build or nix-copy-clojure. (65 issues)
  * *builder*: errors that occur in a builder, like bash. (2 issues) 

Before revising or adding the error messages themselves, I propose working up a generate error 
format.  [This article](https://elm-lang.org/news/compiler-errors-for-humans) outlines the elm error message approach.

# class 1:  nix language errors.

https://github.com/NixOS/nix/issues/3088
https://github.com/NixOS/nix/issues/3063

- nix language errors are the most similar to elm errors.
- use error template as below.
- build an error message catalog.
  collection of *.nix snippets that produce errors.

Language error template:

      ---<Error Type>----------------------------- <filename>

      general error text.

      <line number> line of code where error occurred, with color underlining.
                         ^^ 

      hint about how to fix the problem, perhaps with text indicating the problem:
        which occurred at this word: <of>
        
# class 2: builtin errors.

example:  https://github.com/NixOS/nix/issues/2431

These are errors that occur in the builtin functions.  The error format is similar to the language
errors, but the errors themselves may come from software external to nix, like git.
We may want to detect and interpret errors that these programs return in the context of nix usage.

      ---<Error Type>----------------------------- <filename>

      general error text, or error returned by an external program.  What happened?
          fatal: not a tree object
          error: program 'git' failed with exit code 128

      <line number> builtin.function in nix code where error occurred, with color underlining.
                    ^^^^^^^^^^^^^^^^
      hint about how to fix the problem, perhaps with text indicating the problem:
        This is likely because of an invalid rev, or a rev that is not in the specified ref.
        fetchGit fetches the entire history of the specified ref, and the specified rev must be an ancestor of that.


# class 3:  tool error messages.

warnings and errors from nix tools like nix-copy-closure, nix-collect-garbage, nix-instantiate, etc.

most of the tool issues are requests for warnings when certain conditions hold.

warning example: https://github.com/NixOS/nix/issues/1492

error example: https://github.com/NixOS/nix/issues/765

- tools may evaluate nix expressions, in which case the errors will be in the language format.
- Tool errors in the nix issues often require special code changes to detect problems.

warning mode.

error template:

      --<Error type>------------------------------- <tool name>

      general error text.
      
      tool command line, ie nix-collect-garbage -d

      proposed solution?

      docs link.


Specific examples!

# class 4:  bash/builder errors.

bash is by far the most common builder.  When there is problem in a bash script, ideally we'd 
like to report the file and line number for that problem.  This isn't something that's natural for bash, but
you can get pretty close with *set -x* together with setting a $PS4 environment variable. 

For an individual package, you can enable this with by adding these attributes:

  NIX_DEBUG=6; 
  PS4="\${BASH_SOURCE}:\${LINENO} ";
 
That's great if the problem is in your top level package, but what if its a few packages down?
You'd have to clone nixpkgs and systematically add these flags to every package in the chain
of errors.  I propose enabling these flags in all packages with a sufficient verbosity level.

 
