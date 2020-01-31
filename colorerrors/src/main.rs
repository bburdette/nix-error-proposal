extern crate colored; // not needed in Rust 2018

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
    // level
    let lstring = match einfo.level {
        ErrLevel::Error => "error:".red(),
        ErrLevel::Warning => "warning:".yellow(),
    };
    // divider
    println!(
        "{} {} {} {} {}",
        lstring,
        "---".blue(),
        einfo.err_name.blue(),
        "------------------------".blue(),
        einfo.tool_name.blue()
    );
    println!("");
    // filename
    match &einfo.nix_file {
        Some(fname) => {
            println!("in file: {}", fname.blue());
            println!("");
        }
        None => (),
    }
    // description
    println!("{}", einfo.description);
    println!("");
    // line of code
    match &einfo.err_line {
        Some(eline) => {
            println!("{}: {}", eline.line_no, eline.loc);
            match &eline.column_range {
                Some((col, len)) => println!(
                    "    {}{}",
                    " ".to_string().repeat(*col),
                    "^".to_string().repeat(*len).red()
                ),
                None => (),
            }
        }
        None => (),
    }
    // hint
    println!("{}", einfo.hint);
}

// fn print_error(err_name: &str, tool_name &str, general) {
//   println!( ---<Error Name>----------------------------- <nix tool name>
//       error in file: <nix filename>

//       <general error description>

//       <line number> line of code where error occurred, with color underlining.
//                                                                   ^^^^^^^^^^^
//       hint about how to fix the problem, perhaps with templated text indicating
//       the problem:
//         "underlining" symbol was not found.
// test the example with `cargo run --example most_simple`
fn main() {
    // let x = "blah".green();
    // let y: String = x; // error!
    // TADAA!

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
        hint: "error hint".to_string().green(),
    };

    /*      ***<Error Name>***************************** <nix tool name>
          error in file: <nix filename>

          <general error description>

          <line number> line of code where error occurred, with color underlining.
                                                                      ^^^^^^^^^^^
          hint about how to fix the problem, perhaps with templated text indicating
          the problem:
            "underlining" symbol was not found.

    */
    // struct ErrLine {
    //   line_no: i32,
    //   column_range: Option<(usize,usize)>,
    //   loc: String,
    // }

    print_error(&generic);

    // println!("{} {} !", "it".green(), "works".blue().bold());
}
