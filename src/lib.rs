#![feature(box_patterns)]

extern crate codespan;
extern crate codespan_reporting;
extern crate fnv;
extern crate lalrpop_util;
extern crate uuid;

pub mod ast;
pub mod lex;
pub mod ty;
pub mod ir;
pub mod error;
pub mod token;
pub mod parse;
