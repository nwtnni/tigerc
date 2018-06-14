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

use tigerc::ast::Exp;
use tigerc::parse::Parser;
use tigerc::lex::TokenStream;
use tigerc::error::Error;
use tigerc::ty::Checker;

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

struct Compiler {
    opts: Opt,
    code: CodeMap,
}

impl Compiler {

    fn new() -> Self {
        Compiler {
            opts: Opt::from_args(),
            code: CodeMap::new(),
        }
    }

    fn lex(diagnostic: bool, source: &FileMap, path: &PathBuf, code: &CodeMap) -> Result<TokenStream, Error> {

        let lexed = TokenStream::from(&*source);

        if diagnostic {
            let output = path.with_extension("lexed");
            let mut outfile = File::create(output).unwrap();

            match &lexed {
            | Ok(stream) => {
                for (start, token, _) in stream.tokens() {
                    let (row, col) = source.location(*start).unwrap();
                    write!(outfile, "{}:{} {}\n", row.number(), col.number(), token).unwrap();
                }
            },
            | Err(err) => write!(outfile, "{}", err.to_debug(code)).unwrap(),
            }
        }

        lexed
    }

    fn parse(diagnostic: bool, lexer: TokenStream, path: &PathBuf, code: &CodeMap) -> Result<Exp, Error> {

        let parser = Parser::new();
        let parsed = parser.parse(lexer);

        if diagnostic {
            let output = path.with_extension("parsed");
            let mut outfile = File::create(output).unwrap();

            match &parsed {
            | Ok(ast)  => write!(outfile, "{}", ast).unwrap(),
            | Err(err) => write!(outfile, "{}", err.to_debug(code)).unwrap(),
            };
        }

        parsed
    }

    fn type_check(diagnostic: bool, ast: Exp, path: &PathBuf, code: &CodeMap) -> Result<(), Error> {

        let checked = Checker::check(&ast);

        if diagnostic {
            let output = path.with_extension("typed");
            let mut outfile = File::create(output).unwrap();

            match &checked {
            | Ok(_)    => write!(outfile, "Valid Tiger Program").unwrap(),
            | Err(err) => write!(outfile, "{}", err.to_debug(code)).unwrap(),
            };
        }
        checked
    }

    fn run(&mut self) -> Result<(), Error> {
        let files = self.opts.files.clone();
        for path in &files {
            let source = self.code.add_filemap_from_disk(&path).unwrap();
            let lexer = Self::lex(self.opts.lex, &*source, path, &self.code)?;
            let ast = Self::parse(self.opts.parse, lexer, path, &self.code)?;
            let _ = Self::type_check(self.opts.type_check, ast, path, &self.code)?;
        }
        Ok(())
    }
}

fn main() {
    let mut compiler = Compiler::new();
    let stdout = StandardStream::stdout(ColorChoice::Always);
    match compiler.run() {
    | Err(err) => emit(stdout, &compiler.code, &err.into()).unwrap(),
    | _        => (),
    };
}
