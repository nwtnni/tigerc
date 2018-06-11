#[macro_use] extern crate structopt;
extern crate codespan;
extern crate codespan_reporting;
extern crate tiger_rs;

use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use codespan::CodeMap;
use codespan_reporting::emit;
use codespan_reporting::termcolor::{StandardStream, ColorChoice};
use structopt::StructOpt;

use tiger_rs::parse::*;
use tiger_rs::lex::*;
use tiger_rs::error::*;

#[derive(Debug, StructOpt)]
#[structopt(name = "tigerc")]
struct Opt {

    /// Write parsing and lexing diagnostics to file.
    #[structopt(short = "p", long = "parse")]
    parse: bool,

    /// Files to compile.
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
}

fn main() -> Result<(), Error> {

    let opt = Opt::from_args();
    let mut stderr = StandardStream::stderr(ColorChoice::Always);
    
    let parser = Parser::new();
    let mut code = CodeMap::new();

    for input in &opt.files {
        
        let infile = code.add_filemap_from_disk(&input).unwrap();
        let lexer = Lexer::new(&infile);
        let result = parser.parse(lexer);

        if opt.parse {
            let output = input.with_extension("parsed");
            let mut outfile = File::create(output).unwrap();
            
            match &result {
            | Err(err) => write!(outfile, "{}", err.to_debug(&code)).unwrap(),
            | Ok(ast)  => write!(outfile, "{}", ast).unwrap(),
            };
        }

        match result {
        | Err(err) => emit(&mut stderr, &code, &err.into()).unwrap(),
        | Ok(ast) => println!("{}", ast),
        };
    }

    Ok(())
}
