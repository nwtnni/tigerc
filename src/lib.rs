#[macro_use]
extern crate count;
extern crate codespan;
extern crate codespan_reporting;
extern crate fnv;
extern crate itertools;
extern crate lalrpop_util;
extern crate sym;

#[macro_use]
pub mod util;
pub mod config;
pub mod error;
pub mod span;

pub mod lex;
pub mod parse;
pub mod check;
pub mod translate;

pub mod token;
pub mod ast;
pub mod ty;
pub mod ir;
pub mod operand;
