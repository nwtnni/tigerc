mod display;
mod grammar;

use lalrpop_util::ParseError;

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
        let len = input.len();
        self.0.parse(input).map_err(|err| {
            if let ParseError::UnrecognizedToken { token: None, expected } = err {
                (ParseError::UnrecognizedToken {
                    token: Some((len - 1, Token::Str("".to_string()), len)),
                    expected
                }).into()
            } else {
                err.into()
            }
        })
    }
}
