use codespan::{ByteIndex, ByteSpan, CodeMap};
use codespan_reporting::{Diagnostic, Label};
use lalrpop_util::ParseError as LalrpopError;

use token::Token;

#[derive(Debug)]
pub struct Error {
    span: ByteSpan,
    kind: Kind,
}

impl Error {
    pub fn to_debug(&self, files: &CodeMap) -> String {
        let file = files.find_file(self.span.start()).unwrap();
        let (row, col) = file.location(self.span.start()).unwrap();

        let category = match self.kind {
        | Kind::Lexical(_)   => "lexical", 
        | Kind::Syntactic(_) => "syntactic",
        | Kind::Semantic(_)  => "semantic",
        };
        
        let message: String = (&self.kind).into();
        format!("{}:{} {} error: {}", row.number(), col.number(), category, message)
    }

    pub fn lexical(start: ByteIndex, end: ByteIndex, err: LexError) -> Self {
        Error { span: ByteSpan::new(start, end), kind: Kind::Lexical(err), }
    }

    pub fn syntactic(start: ByteIndex, end: ByteIndex, err: ParseError) -> Self {
        Error { span: ByteSpan::new(start, end), kind: Kind::Syntactic(err), }
    }

    pub fn semantic(start: ByteIndex, end: ByteIndex, err: TypeError) -> Self {
        Error { span: ByteSpan::new(start, end), kind: Kind::Semantic(err), }
    }
}

impl Into<Diagnostic> for Error {
    fn into(self) -> Diagnostic {
        let Error { span, kind } = self;
        Diagnostic::new_error(&kind).with_label(Label::new_primary(span))
    }
}

#[derive(Debug)]
pub enum Kind {
    Lexical(LexError),
    Syntactic(ParseError),
    Semantic(TypeError),
}

impl <'a> Into<String> for &'a Kind {
    fn into(self) -> String {
        match self {
        | Kind::Lexical(err)   => err.into(),
        | Kind::Syntactic(err) => err.into(),
        | Kind::Semantic(err)  => err.into(),
        }
    }
}

#[derive(Debug)]
pub enum LexError {
    Comment,
    Integer,
    Unknown,
}

impl <'a> Into<String> for &'a LexError {
    fn into(self) -> String {
        match self {
        | LexError::Comment => "Comments must begin with [/*].".to_string(),
        | LexError::Integer => "Integers must be between −2,147,483,648 and 2,147,483,647.".to_string(),
        | LexError::Unknown => "Unknown token.".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum ParseError {
    Extra,
    Unexpected,
}

#[derive(Debug)]
pub enum TypeError {}

impl Into<Error> for LalrpopError<ByteIndex, Token, Error> {
    fn into(self) -> Error {
        match self {
        | LalrpopError::InvalidToken { .. }                   => panic!("Internal error: should be covered by custom lexer"),
        | LalrpopError::ExtraToken { token: (start, _, end) } => Error::syntactic(start, end, ParseError::Extra),
        | LalrpopError::User { error }                        => error,
        | LalrpopError::UnrecognizedToken { token, .. }       => {
            match token {
            | None => panic!("Internal error: should be covered by parser"),
            | Some((start, _, end)) => Error::syntactic(start, end, ParseError::Unexpected),
            }
        },
        }
    }
}

impl <'a> Into<String> for &'a ParseError {
    fn into(self) -> String {
        match self {
        | ParseError::Extra      => "Extra tokens encountered.".to_string(),
        | ParseError::Unexpected => "Unexpected token encountered.".to_string(),
        }
    }
}

impl <'a> Into<String> for &'a TypeError {
    fn into(self) -> String {
        String::new()
    }
}
