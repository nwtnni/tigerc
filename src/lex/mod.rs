mod lexer;

use codespan::{ByteIndex, FileMap};

use self::lexer::Lexer;
use error::Error;
use token::Token;

pub type Spanned = (ByteIndex, Token, ByteIndex);

pub struct TokenStream(Result<Vec<Spanned>, Error>);

impl TokenStream {
    pub fn new(source: &FileMap) -> Self {
        let mut lexer = Lexer::new(source);
        let mut tokens = Vec::new();

        loop {
            match lexer.next() {
            | Some(Ok(token)) => tokens.push(token),
            | Some(Err(err))  => return TokenStream(Err(err)),
            | None            => return TokenStream(Ok(tokens)),
            };
        }
    }
}

impl IntoIterator for TokenStream {

    type Item = Result<Spanned, Error>;
    type IntoIter = ::std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        match self.0 {
        | Ok(stream) => stream.into_iter().map(|token| Ok(token)).collect::<Vec<_>>().into_iter(),
        | Err(err) => vec![Err(err)].into_iter(),
        }
    }
}

impl <'read> IntoIterator for &'read TokenStream {

    type Item = Result<&'read Spanned, &'read Error>;
    type IntoIter = ::std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        match &self.0 {
        | Ok(stream) => stream.into_iter().map(|token| Ok(token)).collect::<Vec<_>>().into_iter(),
        | Err(err) => vec![Err(err)].into_iter(),
        }
    }
}
