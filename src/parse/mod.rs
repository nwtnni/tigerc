mod grammar;

use ast::Exp;
use lex::TokenStream;
use error::Error;

/// Wrapper type around generated parser.
pub struct Parser(grammar::ProgramParser);

impl Parser {

    pub fn new() -> Self {
        Parser(grammar::ProgramParser::new())
    }

    pub fn parse(&self, input: TokenStream) -> Result<Exp, Error> {
        self.0.parse(input).map_err(|err| err.into())
    }
}
