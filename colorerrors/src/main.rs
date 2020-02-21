extern crate colored;

use colored::*;

struct ErrInfo {
    level: ErrLevel,
    err_name: String,
    tool_name: String,
    description: String,
    nix_file: Option<String>,
    err_line: Option<ErrLine>,
    hint: ColoredString,
}

struct ErrLine {
    line_no: i32,
    column_range: Option<(usize, usize)>,
    loc: String,
}

enum ErrLevel {
    Warning,
    Error,
}

fn print_error(einfo: &ErrInfo) {
    let errwidth: usize = 80;
    let prefix = "  ";

    // level
    let lstring = match einfo.level {
        ErrLevel::Error => "error:".red(),
        ErrLevel::Warning => "warning:".yellow(),
    };

    let dashwidth = {
        let ndl = lstring.len() + 3 + einfo.err_name.len() + einfo.tool_name.len();
        if ndl > errwidth - 3 {
            3
        } else {
            80 - ndl
        }
    };

    // divider
    println!(
        "{}{} {} {} {} {}",
        prefix,
        lstring,
        "---".blue(),
        einfo.err_name.blue(),
        "-".repeat(dashwidth).blue(),
        einfo.tool_name.blue()
    );

    // filename
    match &einfo.nix_file {
        Some(fname) => {
            println!("{}in file: {}", prefix, fname.blue());
            println!("{}", prefix);
        }
        None => {
            println!("{}from command line argument", prefix);
            println!("{}", prefix);
        }
    }

    // description
    println!("{}{}", prefix, einfo.description);
    println!("{}", prefix);

    // line of code
    match &einfo.err_line {
        Some(eline) => {
            println!("{}{}: {}", prefix, eline.line_no, eline.loc);
            match &eline.column_range {
                Some((col, len)) => println!(
                    "{}    {}{}",
                    prefix,
                    " ".to_string().repeat(*col),
                    "^".to_string().repeat(*len).red()
                ),
                None => (),
            }
        }
        None => (),
    }

    // hint
    println!("{}{}", prefix, einfo.hint);
    println!("{}", prefix);
}

fn main() {
    let generic = ErrInfo {
        level: ErrLevel::Error,
        err_name: "error name".to_string(),
        tool_name: "nix tool name".to_string(),
        description: "general error description".to_string(),
        nix_file: Some("myfile.nix".to_string()),
        err_line: Some(ErrLine {
            line_no: 7,
            column_range: Some((22, 14)),
            loc: "line of code where the error occurred".to_string(),
        }),
        hint: format!(
            "error hint with templated {}",
            "values".to_string().yellow()
        )
        .white(),
    };

    let langwarning = ErrInfo {
        level: ErrLevel::Warning,
        err_name: "Attribute Name".to_string(),
        tool_name: "nix-build".to_string(),
        description: "Attribute format is incorrect.  Only letters a-z, A-Z, 0-9, or one of \"+_-\" are allowed.".to_string(),
        nix_file: None,
        err_line: Some(ErrLine {
            line_no: 1,
            column_range: Some((2,8)),
            loc: "{ \"hi.there\" = (import <nixpkgs> {}).hello; }".to_string(),
        }),
        hint: format!("The symbol {} doesn't satisfy attribute naming requirements.  It will be ignored.", "hi.there".yellow()).to_string().white(),
    };

    let langerror = ErrInfo {
        level: ErrLevel::Error,
        err_name: "String Error".to_string(),
        tool_name: "nix-build".to_string(),
        description: "Invalid escape character.  Only \\t \\n \\r \\\\ are allowed.".to_string(),
        nix_file: None,
        err_line: Some(ErrLine {
            line_no: 1,
            column_range: Some((13, 2)),
            loc: "{ foo = \"test \\e\"; }".to_string(),
        }),
        hint: format!(
            "{} is an invalid escape character for a nix string.",
            "\\e".yellow()
        )
        .to_string()
        .white(),
    };

    let builtinerror = ErrInfo {
        level: ErrLevel::Error,
        err_name: "fetchGit Error".to_string(),
        tool_name: "nix build".to_string(),
        description: "builtin.fetchGit returned an error.".to_string(),
        nix_file: Some("default.nix".to_string()),
        err_line: Some(ErrLine {
            line_no: 101,
            column_range: Some((1, 16)),
            loc: "builtin.fetchGit {".to_string(),
        }),
        hint: "fetchGit takes 4 arguments in a nix expression:
      { url 
      , rev (optional)
      , ref (optional)
      , name (optional)
      }
  If the ref (branch) does not contain the rev (commit), then the fetch will fail.

  See the manual for more:

  https://nixos.org/nix/manual/#builtin-fetchGit"
            .white(),
    };

    let toolwarning = ErrInfo {
        level: ErrLevel::Warning,
        err_name: "user-only garbage collection".to_string(),
        tool_name: "nix-collect-garbage".to_string(),
        description: "collecting garbage for user account only.".to_string(),
        nix_file: None,
        err_line: None,
        hint: format!(
            "this command will collect garbage for user {} only.
  To remove OS generations, run as root or use sudo.  See the manual for more.
  https://nixos.org/nix/manual/#name-5",
            "bburdette".yellow(),
        )
        .white(),
    };

    let toolerror = ErrInfo {
        level: ErrLevel::Error,
        err_name: "remote builder error".to_string(),
        tool_name: "nix-store".to_string(),
        description: "the remote builder doesn't meet package requirements.".to_string(),
        nix_file: None,
        err_line: None,
        hint: format!(
            "This build has requiredSystemFeatures: {}
  But the remote machine only has: {}.
  If the remote machine has the features, declare them in its builder specification.

  See the nix manual for more information:
  https://nixos.org/nix/manual/#chap-distributed-builds",
            "[ \"big-parallel\" ]".yellow(),
            "[]".yellow(),
        )
        .white(),
    };

    // print all the errors
    println!("");
    print_error(&generic);
    print_error(&langwarning);
    print_error(&langerror);
    print_error(&builtinerror);
    print_error(&toolerror);

    // tool warning gets context.
    println!(
        "{}:{}$ nix-collect-garbage",
        "bburdette@BB-5520".green(),
        "~/code/deploy-bots".blue()
    );
    print_error(&toolwarning);
    println!("finding garbage collector roots...");
    println!("deleting garbage...");
    println!("deleting '/nix/store/trash'");
    println!("deleting unused links...");
    println!("note: currently hard linking saves 932.01 MiB");
    println!("0 store paths deleted, 0.00 MiB freed");
}
