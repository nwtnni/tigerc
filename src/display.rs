use std::fmt;
use std::cell::Cell;

use ast::*;

struct Printer<'program> {
    indent: Cell<usize>, 
    program: &'program [Dec],
}

// impl<'program> fmt::Display for Printer<'program> {

//     fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {

//     }

// }
