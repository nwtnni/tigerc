use lex::LexError;
use parse::ParseError;

#[derive(Debug)]
pub enum Error {
    Lexical(LexError),
    Syntactic(ParseError),
    
}

impl From<LexError> for Error {
    fn from(err: LexError) -> Self { Error::Lexical(err) }
}

impl From<ParseError> for Error {
    fn from(err: ParseError) -> Self { Error::Syntactic(err) }
}
