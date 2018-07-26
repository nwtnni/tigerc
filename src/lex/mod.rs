mod lexer;

use std::fmt;
use std::sync::Arc;

use codespan::{ByteIndex, FileMap};

use self::lexer::Lexer;
use error::Error;
use token::Token;

pub fn lex(source: Arc<FileMap>) -> Result<TokenStream, Error> {
    let mut tokens = Vec::new();
    {
        let mut lexer = Lexer::new(&*source);
        loop {
            match lexer.next() {
            | Some(Ok(token)) => tokens.push(token),
            | Some(Err(err))  => return Err(err),
            | None            => break,
            };
        }
    }

    Ok(TokenStream(tokens, source))
}

pub type Spanned = (ByteIndex, Token, ByteIndex);

pub struct TokenStream(Vec<Spanned>, Arc<FileMap>);

impl IntoIterator for TokenStream {

    type Item = Result<Spanned, Error>;
    type IntoIter = ::std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter().map(|token| Ok(token)).collect::<Vec<_>>().into_iter()
    }
}

impl fmt::Display for TokenStream {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {

        for (start, token, _) in &self.0 {
            let (row, col) = self.1.location(*start)
                .expect("Internal error: missing location");

            write!(fmt, "{}:{} {}\n", row.number(), col.number(), token)
                .expect("Internal error: IO");
        }

        Ok(())
    }
}
