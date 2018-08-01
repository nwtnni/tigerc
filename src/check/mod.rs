pub mod context;
mod check;
mod escape;

use ir;
use ast;
use error;

pub fn check(mut ast: ast::Exp) -> Result<ir::Unit, error::Error> {
    self::check::Checker::check(&mut ast)
}
