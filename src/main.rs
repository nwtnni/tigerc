#[macro_use] extern crate structopt;
extern crate codespan;
extern crate failure;
extern crate tiger_rs;

use codespan::CodeMap;
use failure::Error;
use structopt::StructOpt;

use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;

use tiger_rs::parse::*;

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
    
    let parser = ProgramParser::new();
    let mut code = CodeMap::new();

    for path in &opt.files {
        
        let file = code.add_filemap_from_disk(path).unwrap();
        let parsed = parser.parse(file.src());

        match parsed {
        | Err(err) => println!("{:?}", err),
        | Ok(program) => println!("{}", program),
        };
    }

    Ok(())
}
