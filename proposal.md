

# Nix Error Enhancement

One of the areas where Nix could have increased ease of use is in error quality.

In the past decade there has been a trend towards increased ease of use for development tools.  The Elm language is one of the leaders in this area, having inspired [helpful errors](https://blog.rust*lang.org/2016/08/10/Shape*of*errors*to*come.html) in other projects such as Rust.  [This article](https://elm*lang.org/news/compiler*errors*for*humans) by the author of Elm outlines their basic approach.

To my mind, the main goal of these enhanced errors is to minimize the time that the nix user must spend to correct their problem.  Ideally the error should provide all the information needed to fix things and move on, without having to resort to the docs, online help, or grepping through nixpkgs.  If an error message can't provide a solution, at least it should indicate where the problem occurred and where to look for more information.

More precisely, errors ought to: 
* Have a consistent error format that's easily recognizable by the user, with the same types of data in the same places every time.
* Have a clear divider to show where the error report begins, and to separate multiple errors from each other.
* Be easy to see amid a wash of log data or debug output.
* Show the file, line and column of the code where the error occurred, whenever possible.
* Show the actual text of the offending code in the error itself.
* Use color in the format to make it a quicker to parse visually.
* Provide a general description of the error.
* Provide a suggested course of action to correct the problem.
* Provide concrete examples of the problem and potential solutions.

## Error Template

Errors messages should, as much as possible, share a common format for consistency.   For nix language errors, an error template would look something like this:

    error/warning: --- <error name> ----------------------------------- \<nix tool name>
    in file: <nix filename>
    
    <general error description>
    
    <line number>: <nix code containing the error>
                              
    <error hint>

And rendered in the terminal, with colored text:

![generic error](https://bots.practica.site/static/nixerr-imgs/generic.png)

For different categories of error, the rendered error will contain different information.  Speaking of which:

## Categories of nix errors

For inspiration - and concrete examples - I've gone through the nix issue database on github and flagged issues that are candidates for error message improvement.  There are about 90 issues, and they fall roughly into four categories:

  * **language**: syntax errors and the like in the nix language. (13 issues)
  * **builtin**: errors that occur in builtin functions. (6 issues)
  * **tool**: non-language  errors returned by tools like nix-build or nix-copy-clojure. (65 issues)
  * **builder**: errors that occur in a builder, like bash. (2 issues) 

These are available in this [spreadsheet](https://docs.google.com/spreadsheets/d/1YeMT8nQPaMaZWLKE0IqVY5o8XvfiNbhuv1TWWZ0VwJk/edit#gid=1201267462) with the candidate errors at the top.

## Language Errors

Language errors will make use of the full error template.  There may be a nix file that contains the error, and a line of code where the error occurred.  

### language warning example:

[Issue 3088](https://github.com/NixOS/nix/issues/3088) nix-build ignores attributes with a period {"a.b" = drv}

This works:

	$ nix-build --expr '{ hi = (import <nixpkgs> {}).hello; }'
	/nix/store/...-hello-2.10


But this:

	$ nix-build --expr '{ "hi.there" = (import <nixpkgs> {}).hello; }'

Should emit:
![language warning](https://bots.practica.site/static/nixerr-imgs/attributename.png)
Since there was a problem in a nix expression, this example includes the line number and line of code where the problem occurred.

### language error example:
https://github.com/NixOS/nix/issues/3063
Nix should fail (or warn) on unsupported string escapes

	$ nix-build --expr '{ foo = "test \e"; }'

![enter image description here](https://bots.practica.site/static/nixerr-imgs/escapechar.png)        
## Builtin Errors

The error format for builtin errors is the same as the language errors, but the errors themselves may come from software external to nix, like git.  We may want to detect or interpret errors that these programs return in the context of nix usage.  An example:

[Issue 2431](https://github.com/NixOS/nix/issues/2431) fetchGit fails with a not very helpful error message when fetching a revision not in the remote's HEAD.

Current output:

```
[nix-shell:~/code/nix-errors-wk/colorerrors]$ cat << EOF | nix repl
> builtins.fetchGit {
>   url = https://github.com/nixos/nixpkgs-channels;
>   rev = "01f5e794913a18494642b5f237bd76c054339d61";
> }
> EOF
Welcome to Nix version 2.3.2. Type :? for help.

fatal: not a tree object: 01f5e794913a18494642b5f237bd76c054339d61
error: program 'git' failed with exit code 128
```

![fetchgit image](https://bots.practica.site/static/nixerr-imgs/fetchgit.png)
##  Tool Errors

These are warnings and errors from nix tools like nix-copy-closure, nix-collect-garbage, nix-instantiate, etc.  In the github issue database, most of the tool issues are requests for warnings when certain conditions hold.  In most cases the heavyweight error format used for language/builtin errors is not needed.  

#### warning example: 
[Issue 1492](https://github.com/NixOS/nix/issues/1492) nix-collect-garbage -d should issue a warning when run as non-root user

Current output:
```
bburdette@BB-5520:~/code/deploy-bots$ nix-collect-garbage 
finding garbage collector roots...
deleting garbage...
deleting '/nix/store/trash'
deleting unused links...
note: currently hard linking saves 932.01 MiB
0 store paths deleted, 0.00 MiB freed
```

Proposed output:

![fetchgit image](https://bots.practica.site/static/nixerr-imgs/garbage.png)

This is admittedly pretty heavyweight output for many warning messages.
For new users, it would be helpful though.  Lowering the verbosity level below the default
could reduce the output. 


#### error example: 
[Issue 2238](https://github.com/NixOS/nix/issues/2238) Improve error message when missing remote building feature forbids build

```
nix-store --realise --builders 'ssh://root@1.2.3.4 x86_64-linux' -j0 /nix/store/i0kwyxpihg1gcp9jg4qwp7qcrpagj818-chromium-67.0.3396.87.drv /nix/store/bmigs53iryqpqjsy5w4qjfndlh6hxbms-chromium-67.0.3396.87.drv
```
yields 

`unable to start any build; either increase '--max-jobs' or enable remote builds`

Proposed output:

![fetchgit image](https://bots.practica.site/static/nixerr-imgs/remote-builder.png)

### class 4:  bash/builder errors.

bash is by far the most common builder.  When there is a problem in a bash script, ideally we'd 
like to report the file and line number for that problem.  This isn't something that's natural for bash, but
you can get pretty close with *set -x* together with setting a $PS4 environment variable. 

For an individual package, you can enable this with by adding these attributes:

  NIX_DEBUG=6; 
  PS4="\${BASH_SOURCE}:\${LINENO} ";
 
That's great if the problem is in your top level package, but what if its a few packages down?
You'd have to clone nixpkgs and systematically add these flags to every package in the chain
of errors.  I propose enabling these flags in all packages with a sufficient verbosity level.

When there is a bash error, propose common debugging strategies for bash builders.

# Implementation

First is to implement an error printing function in the C code.  I have a rust mockup [here](https://github.com/bburdette/nix-errors-wk/tree/master/colorerrors).  That's the easy part!

Next up is to make sure the information needed for the standard error format is available at error time, in the nix language processing, and anywhere that error messages can be produced.  

With the error format information available, we need to provide all that for each of the **language** and **builtin** errors in nix - hints, error templates, error name, general error description.

For builtins, specialized logic may be needed to interpret results from external tools like git or curl.  General hints and strategy for these kinds of errors will be helpful too.

The **tool** errors are probably the most challenging, if we really want to address all the issues in the nix github.  Most of these would require dedicated C code to detect special conditions, in addition to the error hints, general description, and etc.  Each of these is different!

Lastly the **builder** errors.  Focusing on bash seems like the best bang for the buck.  Getting the file and line information for bash errors should go a long way towards making them more debuggable.  



