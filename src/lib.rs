#![feature(box_patterns)]


#[macro_use]
extern crate simple_counter;
extern crate simple_symbol;
extern crate codespan;
extern crate codespan_reporting;
extern crate fnv;
extern crate itertools;
extern crate lalrpop_util;
extern crate petgraph;

#[macro_use]
pub mod util;
pub mod config;
pub mod error;
pub mod span;

pub mod lex;
pub mod parse;
pub mod check;
pub mod translate;
pub mod interpret;
pub mod assemble;

pub mod token;
pub mod ast;
pub mod ty;
pub mod ir;
pub mod asm;
pub mod operand;
pub mod unit;
