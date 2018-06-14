mod lexer;

use codespan::{ByteIndex, FileMap};

use self::lexer::Lexer;
use error::Error;
use token::Token;

pub type Spanned = (ByteIndex, Token, ByteIndex);

pub struct TokenStream(Vec<Spanned>);

impl TokenStream {
    pub fn from(source: &FileMap) -> Result<Self, Error> {
        let mut lexer = Lexer::new(source);
        let mut tokens = Vec::new();

        loop {
            match lexer.next() {
            | Some(Ok(token)) => tokens.push(token),
            | Some(Err(err))  => return Err(err),
            | None            => return Ok(TokenStream(tokens)),
            };
        }
    }

    pub fn tokens(&self) -> &[Spanned] {
        &self.0
    }
}

impl IntoIterator for TokenStream {

    type Item = Result<Spanned, Error>;
    type IntoIter = ::std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter().map(|token| Ok(token)).collect::<Vec<_>>().into_iter()
    }
}
