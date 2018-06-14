use codespan::{ByteIndex, ByteSpan, CodeMap};
use codespan_reporting::{Diagnostic, Label};
use lalrpop_util::ParseError as LalrpopError;

use token::Token;

#[derive(Debug)]
pub struct Error {
    pub span: ByteSpan,
    pub kind: Kind,
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

    pub fn semantic(span: ByteSpan, err: TypeError) -> Self {
        Error { span, kind: Kind::Semantic(err), }
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
pub enum TypeError {
    Break,

    CallMismatch,
    UnboundFun,
    NotFun,
    ReturnMismatch,

    Neg,

    BinaryMismatch,

    UnboundRecord,
    NotRecord,
    FieldMismatch,

    UnusedExp,

    UnboundVar,
    NotVar,
    VarMismatch,

    GuardMismatch,
    BranchMismatch,
    UnusedBranch,

    UnusedWhileBody,

    ForBound,
    UnusedForBody,

    UnboundArr,
    NotArr,
    ArrMismatch,

    UnboundType,

    UnboundField,
    
    IndexMismatch,

    UnknownNil,
}

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
        match self {
        | TypeError::Break            => "Cannot break outside of a loop.".to_string(),
        | TypeError::CallMismatch     => "Wrong arguments to function.".to_string(),
        | TypeError::UnboundFun       => "Could not find function.".to_string(),
        | TypeError::NotFun           => "Not a function.".to_string(),
        | TypeError::ReturnMismatch   => "Function return type doesn't match body.".to_string(),
        | TypeError::Neg              => "Can only negate integers.".to_string(),
        | TypeError::BinaryMismatch   => "Wrong arguments for binary operator.".to_string(),
        | TypeError::UnboundRecord    => "Could not find record.".to_string(),
        | TypeError::NotRecord        => "Not a record.".to_string(),
        | TypeError::FieldMismatch    => "Incorrect type for field.".to_string(),
        | TypeError::UnusedExp        => "Unused expression.".to_string(),
        | TypeError::UnboundVar       => "Could not find variable.".to_string(),
        | TypeError::NotVar           => "Expected variable.".to_string(),
        | TypeError::VarMismatch      => "Incorrect type for variable.".to_string(),
        | TypeError::GuardMismatch    => "Guard expression must be an integer.".to_string(),
        | TypeError::BranchMismatch   => "Branches must return the same type.".to_string(),
        | TypeError::UnusedBranch     => "If branches must return unit.".to_string(),
        | TypeError::UnusedWhileBody  => "While body must return unit.".to_string(),
        | TypeError::ForBound         => "For bounds must be integers.".to_string(),
        | TypeError::UnusedForBody    => "For body must return unit.".to_string(),
        | TypeError::UnboundArr       => "Could not find array.".to_string(),
        | TypeError::NotArr           => "Not an array.".to_string(),
        | TypeError::ArrMismatch      => "Array initializer doesn't match array type.".to_string(),
        | TypeError::UnboundType      => "Could not find type.".to_string(),
        | TypeError::UnboundField     => "Unbound record field.".to_string(),
        | TypeError::IndexMismatch    => "Array indices must be integers.".to_string(),
        | TypeError::UnknownNil       => "Cannot infer type for nil.".to_string(),
        }
    }
}
