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
use tigerc::check::Checker;
use tigerc::translate::{Unit, canonize_ast, fold_ast};

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

    fn type_check(diagnostic: bool, ast: &mut Exp, path: &PathBuf, code: &CodeMap) -> Result<Vec<Unit>, Error> {

        let checked = Checker::check(ast);

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

    fn translate(units: Vec<Unit>) {
        for unit in units {
            println!("{}", unit.label);
            println!("{}", fold_ast(&canonize_ast(unit.body)));
        }
    }

    fn run_once(&mut self, path: &PathBuf) -> Result<(), Error> {
        let source = self.code.add_filemap_from_disk(&path).unwrap();
        let lexer = Self::lex(self.opts.lex, &*source, path, &self.code)?;
        let mut ast = Self::parse(self.opts.parse, lexer, path, &self.code)?;
        let ir = Self::type_check(self.opts.type_check, &mut ast, path, &self.code)?;
        let _ = Self::translate(ir);
        Ok(())
    }

    fn run(&mut self) -> Result<(), Vec<Error>> {
        let files = self.opts.files.clone();
        let mut errors = Vec::new();
        for path in &files {
            if let Err(err) = self.run_once(path) {
                errors.push(err);
            }
        }
        return if errors.is_empty() { Ok(()) } else { Err(errors) }
    }
}

fn main() {
    let mut compiler = Compiler::new();
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    match compiler.run() {
    | Err(errors) => for err in errors { emit(&mut stdout, &compiler.code, &err.into()).unwrap() },
    | _        => (),
    };
}
