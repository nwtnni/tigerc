use lalrpop_util::ParseError;
use token::Token;

#[derive(Debug)]
pub struct Error {
    span: (usize, usize),
    kind: Kind,
}

impl Error {
    pub fn lexical(start: usize, end: usize, err: Lex) -> Self {
        Error { span: (start, end), kind: Kind::Lexical(err), }
    }

    pub fn syntactic(start: usize, end: usize, err: Parse) -> Self {
        Error { span: (start, end), kind: Kind::Syntactic(err), }
    }

    pub fn semantic(start: usize, end: usize, err: Type) -> Self {
        Error { span: (start, end), kind: Kind::Semantic(err), }
    }
}

#[derive(Debug)]
pub enum Kind {
    Lexical(Lex),
    Syntactic(Parse),
    Semantic(Type),
}

#[derive(Debug)]
pub enum Lex {
    Comment,
    Integer,
    Unknown,
}

impl Into<String> for Lex {

    fn into(self) -> String {
        match self {
        | Lex::Comment => "Comments must begin with [/*].".to_string(),
        | Lex::Integer => "Integers must be between −2,147,483,648 and 2,147,483,647.".to_string(),
        | Lex::Unknown => "Unknown token.".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum Parse {
    Extra,
    Unexpected,
}

#[derive(Debug)]
pub enum Type {

}

impl Into<Error> for ParseError<usize, Token, Error> {
    fn into(self) -> Error {
        match self {
        | ParseError::InvalidToken { .. }                   => panic!("Internal error: should be covered by custom lexer"),
        | ParseError::ExtraToken { token: (start, _, end) } => Error::syntactic(start, end, Parse::Extra),
        | ParseError::User { error }                        => error,
        | ParseError::UnrecognizedToken { token, .. }       => {
            match token {
            | None => panic!("Internal error: should be covered by parser"),
            | Some((start, _, end)) => Error::syntactic(start, end, Parse::Unexpected),
            }
        },
        }
    }
}

impl Into<String> for Parse {
    fn into(self) -> String {
        match self {
        | Parse::Extra      => "Extra tokens encountered.".to_string(),
        | Parse::Unexpected => "Unexpected token encountered.".to_string(),
        }
    }
}

impl Into<String> for Type {
    fn into(self) -> String {
        String::new()
    }
}
