use std::fmt;
use std::fs::File;
use std::io::Write;
use std::mem;
use std::path::PathBuf;
use std::sync::Arc;

use codespan::{CodeMap, FileMap};

use ast;
use ir;
use asm;

use lex;
use parse;
use check;
use translate;
use assemble;
use optimize;

use error::Error;
use operand::{Temp, Reg};

pub enum Item {
    Source(Arc<FileMap>),
    Tokens(lex::TokenStream),
    Syntax(ast::Exp),
    Typed(ir::Unit),
    Intermediate(ir::Unit),
    Abstract(asm::Unit<Temp>),
    Assembly(asm::Unit<Reg>),
}

impl fmt::Display for Item {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
        | Item::Source(_) => panic!("Internal error: printing source"),
        | Item::Tokens(stream) => write!(fmt, "{}", stream),
        | Item::Syntax(ast) => write!(fmt, "{}", ast),
        | Item::Typed(_) => write!(fmt, "Valid Tiger Program"),
        | Item::Intermediate(unit) => write!(fmt, "{}\n\n", unit),
        | Item::Abstract(unit) => write!(fmt, "{}\n\n", unit),
        | Item::Assembly(unit) => write!(fmt, "{}\n\n", unit),
        }
    }
}

pub trait Phase {
    fn process(&self, compiler: &Compiler, input: Item) -> Result<Item, Error>;
}

pub struct Compiler {
    phases: Vec<Box<Phase>>, 
    code: CodeMap,
    path: PathBuf,
}

impl Compiler {
    
    pub fn with_path<T: Into<PathBuf>>(path: T) -> Self {
        Compiler {
            phases: Vec::new(),
            code: CodeMap::default(),
            path: path.into(),
        }
    }

    pub fn with_phase(mut self, phase: Box<Phase>) -> Self {
        self.phases.push(phase);
        self
    }

    pub fn run(&mut self) -> Result<Item, Error> {
        let map = self.code.add_filemap_from_disk(&self.path)
            .expect("Internal error: IO")
            .clone();
        
        let phases = mem::replace(
            &mut self.phases,
            Vec::with_capacity(0)
        );

        phases.into_iter()
            .try_fold(Item::Source(map), |item, phase| {
                phase.process(&self, item)
            })
    }

    pub fn code(&self) -> &CodeMap {
        &self.code
    }

    fn write(&self, ext: &'static str, item: &Result<Item, Error>) {
        let output = self.path.with_extension(ext);
        let mut outfile = File::create(output)
            .expect("Internal error: IO");

        match item {
        | Ok(item) => write!(outfile, "{}", item).expect("Internal error: IO"),
        | Err(err) => write!(outfile, "{}", err.to_debug(&self.code)).expect("Internal error: IO"),
        };
    }
}

macro_rules! impl_phase {
    ($phase:ident, $ext:expr, $item:pat => $result:expr) => {
        impl Phase for $phase {
            fn process(&self, compiler: &Compiler, input: Item) -> Result<Item, Error> {
                if self.1 { return Ok(input) }

                match input {
                | $item => {
                    let result = $result;
                    if self.0 { compiler.write($ext, &result); }
                    result
                }
                | _ => panic!("Internal error: incorrect phase input"),
                }
            }
        }

        impl $phase {
            pub fn new(diagnostics: bool) -> Box<Self> {
                Box::new($phase(diagnostics, false))
            }

            pub fn maybe(diagnostics: bool, disable: bool) -> Box<Self> {
                Box::new($phase(diagnostics, disable))
            }
        }
    }
}

pub struct Lex(pub bool, pub bool);

impl_phase! (Lex, "lexed", Item::Source(source) => {
    lex::lex(source).map(|tokens| Item::Tokens(tokens)) 
});

pub struct Parse(pub bool, pub bool);

impl_phase! (Parse, "parsed", Item::Tokens(tokens) => {
    parse::parse(tokens).map(|ast| Item::Syntax(ast))
});


pub struct Type(pub bool, pub bool);

impl_phase! (Type, "typed", Item::Syntax(ast) => {
    check::check(ast).map(|unit| Item::Typed(unit))
});

pub struct Canonize(pub bool, pub bool);

impl_phase! (Canonize, "canonized", Item::Typed(unit) => {
    Ok(Item::Intermediate(translate::canonize(unit)))
});

pub struct Fold(pub bool, pub bool);

impl_phase! (Fold, "folded", Item::Intermediate(unit) => {
    Ok(Item::Intermediate(translate::fold(unit)))
});

pub struct Reorder(pub bool, pub bool);

impl_phase! (Reorder, "reordered", Item::Intermediate(unit) => {
    Ok(Item::Intermediate(translate::reorder(unit)))
});

pub struct Tile(pub bool, pub bool);

impl_phase! (Tile, "tiled", Item::Intermediate(unit) => {
    Ok(Item::Abstract(assemble::tile(unit)))
});

pub struct Trivial(pub bool, pub bool);

impl_phase! (Trivial, "s", Item::Abstract(unit) => {
    Ok(Item::Assembly(assemble::allocate::<assemble::Trivial>(unit)))
});

pub struct CoalesceAbstract(pub bool, pub bool);

impl_phase! (CoalesceAbstract, "coalesced", Item::Abstract(unit) => {
    Ok(Item::Abstract(optimize::coalesce(unit)))
});

pub struct CoalesceAssembly(pub bool, pub bool);

//TODO: change file extension after adding more passes?
impl_phase! (CoalesceAssembly, "s", Item::Assembly(unit) => {
    Ok(Item::Assembly(optimize::coalesce(unit)))
});
