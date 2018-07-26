#[macro_use] extern crate structopt;
extern crate codespan;
extern crate codespan_reporting;
extern crate tigerc;

use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use codespan::{FileMap, CodeMap};
use codespan_reporting::emit;
use codespan_reporting::termcolor::{StandardStream, ColorChoice};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "tigerc")]
struct Opt {

    /// Write lexing diagnostics to file.
    #[structopt(short = "l", long = "lex")]
    lex: bool,

    /// Write parsing diagnostics to file.
    #[structopt(short = "p", long = "parse")]
    parse: bool,

    /// Write type-checking diagnostics to file.
    #[structopt(short = "t", long = "type")]
    type_check: bool,

    /// Files to compile.
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
}

fn main() {
    println!("Hello, world!");
}
