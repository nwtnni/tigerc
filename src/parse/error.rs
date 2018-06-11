use lalrpop_util;
use lex::LexError;
use token::Token;

#[derive(Debug)]
pub struct ParseError(lalrpop_util::ParseError<usize, Token, LexError>);

impl ParseError {
    
    pub fn new(err: lalrpop_util::ParseError<usize, Token, LexError>) -> Self {
        ParseError(err)
    }

}
