mod display;
mod error;
mod grammar;

use ast::Exp;
use error::Error;

pub use self::error::ParseError;

/// Wrapper type around generated parser.
pub struct Parser(grammar::ProgramParser);

impl Parser {

    pub fn new() -> Self {
        Parser(grammar::ProgramParser::new())
    }

    pub fn parse<'input>(&self, input: &'input str) -> Result<Exp, Error> {
        self.0.parse(input).map_err(|err| ParseError::new(err).into())
    }

}
