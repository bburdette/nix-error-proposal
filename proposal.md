# Nix Error Enhancement

One of the areas where Nix could have increased ease of use is in error quality.

In the past decade there has been a trend towards increased ease of use for development tools.  The Elm language is one of the leaders in this area, having inspired [helpful errors](https://blog.rust*lang.org/2016/08/10/Shape*of*errors*to*come.html) in other projects such as Rust.  [This article](https://elm*lang.org/news/compiler*errors*for*humans) by the author of Elm outlines their basic approach.

The main goal of these enhanced errors is to minimize the time that the nix user must spend to correct their problem.  Ideally the error should provide all the information needed to fix things and move on, without having to resort to the docs, online help, or grepping through nixpkgs.  If an error message can't provide a solution, at least it should indicate where the problem occurred and where to look for more information.

This will involve two main parts: creating an enhanced error format, and creating the corresponding content for each error.

The error format ought to: 
* Have a consistent layout that's easily recognizable by the user, with the same types of data in the same places every time.
* Have a clear divider to show where the error report begins, and to separate multiple errors from each other.
* Be easy to see amid a wash of log data or debug output.
* Show the file, line and column of the code where the error occurred, whenever possible.
* Show the actual text of the offending code in the error itself.
* Show the context of the problem code - preceding and following lines.
* Use color in the format to make it quicker to parse visually.
* Provide a general description of the error.
* Provide a more concrete hint section.
* Provide links to the nix docs for more detail.

Content wise, the main enhancement is the hint section.
This ought to provide additional context for the error - not just what happened, but why.
How does the error relate to my code, and what should I do to fix it?
Concrete examples are helpful here, and templated strings in the hint can make it 
even more immediately relatable, with the strings pulled from the user's own code. 

## Error Template

Errors messages should, as much as possible, share a common format for consistency.   For nix language errors, an error template would look something like this:

    error/warning: --- <error name> ----------------------------------- \<nix tool name>
    in file: <nix filename>  at line: <line number>
    
    <general error description>
        
                 |  <previous line of code>
    <line number>|  <nix code containing the error>
                 |                           ^^^^^
                 |  <next line of code>
                              
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

[Issue 3088:](https://github.com/NixOS/nix/issues/3088) nix-build ignores attributes with a period {"a.b" = drv}

This works:

	$ nix-build --expr '{ hi = (import <nixpkgs> {}).hello; }'
	/nix/store/...-hello-2.10


But this:

	$ nix-build --expr '{ "hi.there" = (import <nixpkgs> {}).hello; }'
	
Should produce a warning at least.  Current output:

![language_warning_before](https://bots.practica.site/static/nixerr-imgs/hi.there.before.png)

Proposed warning output:

![language warning](https://bots.practica.site/static/nixerr-imgs/attributename.png)

Since there was a problem in a nix expression, this example includes the line number and line of code where the problem occurred.

### language error example:

[Issue 3063:](https://github.com/NixOS/nix/issues/3063) Nix should fail (or warn) on unsupported string escapes

Current output - bad character is siliently ignored:

![escape char before image](https://bots.practica.site/static/nixerr-imgs/escape.before.png)        

Proposed error output:

![escape char image](https://bots.practica.site/static/nixerr-imgs/escapechar.png)        
## Builtin Errors

The error format for builtin errors is the same as the language errors, but the errors themselves may come from software external to nix, like git.  We may want to detect or interpret errors that these programs return in the context of nix usage.  An example:

[Issue 2431:](https://github.com/NixOS/nix/issues/2431) fetchGit fails with a not very helpful error message when fetching a revision not in the remote's HEAD.

Current output:

![fetchgit image](https://bots.practica.site/static/nixerr-imgs/fetchgit-before.png)

Proposed output:

![fetchgit image](https://bots.practica.site/static/nixerr-imgs/fetchgit.png)

##  Tool Errors

These are warnings and errors from nix tools like nix-copy-closure, nix-collect-garbage, nix-instantiate, etc.  In the github issue database, most of the tool issues are requests for warnings when certain conditions hold.  

#### warning example: 
[Issue 1492:](https://github.com/NixOS/nix/issues/1492) nix-collect-garbage -d should issue a warning when run as non-root user

Current output:

![garbage before image](https://bots.practica.site/static/nixerr-imgs/garbage-before.png)

Proposed output:

![garbage image](https://bots.practica.site/static/nixerr-imgs/garbage.png)

This is admittedly pretty heavyweight output for many warning messages.
For new users, it would be helpful though.  Lowering the verbosity level below the default
could reduce the output.


#### error example: 
[Issue 2238:](https://github.com/NixOS/nix/issues/2238) Improve error message when missing remote building feature forbids build

```
nix-store --realise --builders 'ssh://root@1.2.3.4 x86_64-linux' -j0 /nix/store/i0kwyxpihg1gcp9jg4qwp7qcrpagj818-chromium-67.0.3396.87.drv /nix/store/bmigs53iryqpqjsy5w4qjfndlh6hxbms-chromium-67.0.3396.87.drv
```

current output:

![fetchgit image](https://bots.practica.site/static/nixerr-imgs/chromium.fail.png)

Proposed output:

![fetchgit image](https://bots.practica.site/static/nixerr-imgs/remote-builder.png)

### class 4:  bash/builder errors.

Debugging bash and other builders is outside the scope of this project.  But we can look at adding error messages that point
at helpful documentation when builder errors occur.

# Implementation

First is to implement an error printing function in the C code.  I have a rust mockup [here](https://github.com/bburdette/nix-errors-wk/tree/master/colorerrors).  That's the easy part!

Next up is to make sure the error context information needed for the standard error format is available at error time, in the nix language processing, and anywhere else that error messages can be produced.  Filename, line number, current line of code.  

With the error context information available, we need to provide all that for each of the **language** and **builtin** errors in nix as well as hints, error templates, error name, general error description.

For builtins, specialized logic may be needed to interpret results from external tools like git or curl.  General hints and strategy for these kinds of errors will be helpful too.

The **tool** errors are probably the most challenging, if we really want to address all the issues in the nix github.  Most of these would require dedicated C code to detect special conditions, in addition to the error hints, general description, and etc.  Each of these is different!

Lastly the **builder** errors.  We'll provide some basic links and information to help with debugging.  Going beyond that - like writing a bash debugger, for instance - is outside the scope of this project.
