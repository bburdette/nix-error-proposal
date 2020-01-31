extern crate colored; // not needed in Rust 2018

use colored::*;


struct ErrInfo {
  err_name: String,
  tool_name: String,
  description: String,
  nix_file: Option<String>,
  err_line: Option<ErrLine>,
  hint: ColoredString,
}

struct ErrLine {
  line_no: i32,
  column_range: Option<(usize,usize)>,
  loc: String,
}


fn print_error(einfo: &ErrInfo) {
  println!("---{}----------------------------- {}", einfo.err_name, einfo.tool_name);
  match &einfo.nix_file {
    Some(fname) => println!("error in file: {}", fname),
    None => (),
  }
  match &einfo.err_line {
    Some(eline) => {
      println!("{}: {}", eline.line_no, eline.loc);
      match &eline.column_range {
        Some((col,len)) =>
          println!("    {}{}", " ".to_string().repeat( *col), "^".to_string().repeat( *len).red()),
        None => (),
      }
    }
    None => (),
  }

  println!("{}", einfo.description);
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

  let e = ErrInfo {
      err_name: "error name".to_string(),
      tool_name: "tool name".to_string(),
      description: "error description".to_string(),
      nix_file: Some("myfile.nix".to_string()),
      err_line: Some(ErrLine { 
                              line_no: 7,
                              column_range: Some((22,14)),
                              loc: "line of code where the error occurred".to_string(), }),
      hint: "error hint".to_string().green(),
    };

    // struct ErrLine {
    //   line_no: i32,
    //   column_range: Option<(usize,usize)>,
    //   loc: String,
    // }

  print_error(&e);
  

  // println!("{} {} !", "it".green(), "works".blue().bold());
}
