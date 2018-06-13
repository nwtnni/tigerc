mod display;
mod grammar;

use lalrpop_util::ParseError as LalrpopError;

use ast::Exp;
use lex::Lexer;
use token::Token;
use error::Error;

/// Wrapper type around generated parser.
pub struct Parser(grammar::ProgramParser);

impl Parser {

    pub fn new() -> Self {
        Parser(grammar::ProgramParser::new())
    }

    pub fn parse(&self, input: Lexer) -> Result<Exp, Error> {
        let len = input.len() as u32;
        self.0.parse(input).map_err(|err| {
            if let LalrpopError::UnrecognizedToken { token: None, expected } = err {
                (LalrpopError::UnrecognizedToken {
                    token: Some((len.into(), Token::Str("".to_string()), len.into())),
                    expected
                }).into()
            } else {
                err.into()
            }
        })
    }
}
