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

use tigerc::phase::*;

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

    /// Write intermediate canonized IR to file.
    #[structopt(long = "canonize")]
    canonize: bool,

    /// Write constant-folded IR to file.
    #[structopt(long = "fold")]
    fold: bool,

    /// Disable constant folding.
    #[structopt(long = "o-no-cf")]
    disable_fold: bool,

    /// Write intermediate reordered IR to file.
    #[structopt(long = "reorder")]
    reorder: bool, 

    /// Write tiled abstract assembly to file.
    #[structopt(long = "tile")]
    tile: bool,

    /// Files to compile.
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
}

fn main() {
    let opt = Opt::from_args();
    let mut stdout = StandardStream::stdout(ColorChoice::Auto);

    for file in &opt.files {
        let mut compiler = Compiler::with_path(file)
            .with_phase(Lex::new(opt.lex))
            .with_phase(Parse::new(opt.parse))
            .with_phase(Type::new(opt.type_check))
            .with_phase(Canonize::new(opt.canonize))
            .with_phase(Fold::maybe(opt.fold, opt.disable_fold))
            .with_phase(Reorder::new(opt.reorder))
            .with_phase(Tile::new(opt.tile));

        match compiler.run() {
        | Err(err) => emit(&mut stdout, compiler.code(), &err.into()).expect("Internal error: IO"),
        | _ => (),
        }
    }
}
