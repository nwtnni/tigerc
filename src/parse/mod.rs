mod grammar;

use ast::Exp;
use lex::TokenStream;
use error::Error;

pub fn parse(input: TokenStream) -> Result<Exp, Error> {
    let parser = grammar::ProgramParser::new();
    parser.parse(input).map_err(|err| err.into())
}
