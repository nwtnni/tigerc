use std::str::CharIndices;

use token::Token;
use lex::LexError;

pub struct Lexer<'input> {
    mode: Mode,
    chars: CharIndices<'input>,
}

enum Mode {
    Source,
    Comment,
    Failure,
}

impl <'input> Lexer<'input> {

    pub fn new(source: &'input str) -> Self {
        Lexer {
            mode: Mode::Source,
            chars: source.char_indices(),
        }
    }

}

type Spanned = Result<(usize, Token, usize), LexError>;

impl <'input> Iterator for Lexer<'input> {

    type Item = Spanned;

    fn next(&mut self) -> Option<Self::Item> {



        None
    }

}
