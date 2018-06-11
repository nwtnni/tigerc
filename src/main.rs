#[macro_use] extern crate structopt;
extern crate codespan;
extern crate tiger_rs;

use codespan::CodeMap;
use structopt::StructOpt;

use std::path::PathBuf;

use tiger_rs::parse::*;
use tiger_rs::lex::*;
use tiger_rs::error::*;

#[derive(Debug, StructOpt)]
#[structopt(name = "c--")]
struct Opt {

    /// Write parsing diagnostics to file.
    #[structopt(short = "p", long = "parse")]
    parse: bool,

    /// Files to compile.
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
}

fn main() -> Result<(), Error> {

    let opt = Opt::from_args();
    
    let parser = Parser::new();
    let mut code = CodeMap::new();

    for path in &opt.files {
        
        let file = code.add_filemap_from_disk(path).unwrap();
        let lexer = Lexer::new(file.src());
        let parsed = parser.parse(lexer);

        println!("{}", parsed?);
    }

    Ok(())
}
