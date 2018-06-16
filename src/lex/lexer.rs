use std::str::CharIndices;
use std::str::FromStr;

use codespan::{ByteIndex, ByteOffset, FileMap};
use sym;

use lex::Spanned;
use token::Token;
use error::{Error, LexError};
use span::Span;

pub struct Lexer<'input> {
    mode: Mode,
    source: &'input FileMap,
    stream: CharIndices<'input>,
    next: Option<(usize, char)>,
}

enum Mode {
    Source,
    Comment,
}

fn is_symbol(c: char) -> bool {
    match c {
    | '|' | '&' | '>' | '=' | '<' | '/' | '*' | '-' | '+' | '.'
    | '[' | ']' | '{' | '}' | '(' | ')' | ';' | ':' | ',' => true,
    _ => false,
    }
}

fn is_digit(c: char) -> bool {
    c.is_digit(10)
}

fn is_ident_start(c: char) -> bool {
    match c {
    | 'A' ... 'Z' | 'a' ... 'z' => true,
    | _                       => false,
    }
}

fn is_ident(c: char) -> bool {
    match c {
    | 'A' ... 'Z' | 'a' ... 'z' | '0' ... '9' | '_' => true,
    | _                                          => false,
    }
}

fn is_whitespace(c: char) -> bool {
    c.is_whitespace()
}

impl <'input> Lexer<'input> {

    pub fn new(source: &'input FileMap) -> Self {
        let mut stream = source.src().char_indices();
        let next = stream.next();
        Lexer { mode: Mode::Source, source, stream, next }
    }

    fn skip(&mut self) {
        self.next = self.stream.next();
    }

    fn peek(&self) -> Option<(ByteIndex, char)> {
        self.next.map(|(index, c)| {
            let offset = index as i64;
            (self.source.span().start() + ByteOffset(offset), c)
        })
    }

    fn test_peek<F>(&self, condition: F) -> bool where F: FnOnce(char) -> bool {
        match self.peek() {
        | None         => false,
        | Some((_, c)) => condition(c),
        }
    }

    fn slice(&self, start: ByteIndex, end: ByteIndex) -> &'input str {
        &self.source.src_slice(Span::new(start, end)).unwrap()
    }

    fn take_while<F>(&mut self, start: ByteIndex, mut condition: F) -> (ByteIndex, &'input str)
        where F: FnMut(char) -> bool
    {
        self.take_until(start, |c| !condition(c))
    }

    fn take_until<F>(&mut self, start: ByteIndex, mut stop: F) -> (ByteIndex, &'input str)
        where F: FnMut(char) -> bool
    {
        while let Some((end, c)) = self.peek() {
            match stop(c) {
            | true  => return (end, self.slice(start, end)),
            | false => self.skip(),
            }
        }

        let eof = self.source.span().end();
        (eof, self.slice(start, eof))
    }

    fn take_ident(&mut self, start: ByteIndex) -> (ByteIndex, &'input str) {
        let valid = self.peek().map(|(_, c)| is_ident_start(c)).unwrap_or(false);
        if !valid { return (start, "") }
        let (end, _) = self.take_while(start + ByteOffset(1), is_ident);
        (end, self.slice(start, end))
    }

    fn take_int(&mut self, start: ByteIndex) -> (ByteIndex, &'input str) {
        self.take_while(start, is_digit)
    }

    fn take_string(&mut self, start: ByteIndex) -> (ByteIndex, &'input str) {
        if !self.test_peek(|c| c == '"') { return (start, "") }
        self.skip();
        let (end, _) = self.take_until(start, |c| c == '"');
        self.skip();
        (end + ByteOffset(1), self.slice(start, end))
    }

}

fn error(start: ByteIndex, end: ByteIndex, err: LexError) -> Option<Result<Spanned, Error>> {
    Some(
        Err(
            Error::lexical(start, end, err)
        )
    )
}

fn success(start: ByteIndex, end: ByteIndex, token: Token) -> Option<Result<Spanned, Error>> {
    Some(
        Ok(
            (start, token, end)
        )
    )
}

impl <'input> Iterator for Lexer<'input> {

    type Item = Result<Spanned, Error>;

    fn next(&mut self) -> Option<Self::Item> {

        let mut comment_level = 0;

        while let Some((start, c)) = self.peek() {
            
            if is_whitespace(c) { self.skip(); continue }

            match self.mode {
            | Mode::Source => {

                // Look for symbol
                if is_symbol(c) {
                    self.skip(); 
                    let (double, token) = match c {
                    | '|' => (false, Token::LOr),
                    | '&' => (false, Token::LAnd),
                    | '=' => (false, Token::Eq),
                    | '-' => (false, Token::Sub),
                    | '+' => (false, Token::Add),
                    | '.' => (false, Token::Dot),
                    | '[' => (false, Token::LBrace),
                    | ']' => (false, Token::RBrace),
                    | '{' => (false, Token::LBrack),
                    | '}' => (false, Token::RBrack),
                    | '(' => (false, Token::LParen),
                    | ')' => (false, Token::RParen),
                    | ';' => (false, Token::Semicolon),
                    | ',' => (false, Token::Comma),
                    | ':' => if self.test_peek(|c| c == '=') { (true, Token::Assign) } else { (false, Token::Colon) },
                    | '>' => if self.test_peek(|c| c == '=') { (true, Token::Ge) } else { (false, Token::Gt) }
                    | '*' => if self.test_peek(|c| c == '/') { return error(start, start + ByteOffset(2), LexError::Comment) } else { (false, Token::Mul) },
                    | '/' => if self.test_peek(|c| c == '*') { self.mode = Mode::Comment; self.skip(); continue } else { (false, Token::Div) },
                    | '<' => {
                        if self.test_peek(|c| c == '=')      { (true, Token::Le) }
                        else if self.test_peek(|c| c == '>') { (true, Token::Neq) }
                        else                                 { (false, Token::Lt) }
                    },
                    _ => panic!("Internal error in is_symbol function"),
                    };

                    // Successfully lexed symbol
                    let end = if double { self.skip(); start + ByteOffset(2) } else { start + ByteOffset(1) };
                    return success(start, end, token);
                }

                // Otherwise look for identifier
                let (end, ident) = self.take_ident(start);

                // Check for keywords first
                let token = match ident {
                | "type"     => Some(Token::Type),
                | "var"      => Some(Token::Var),
                | "function" => Some(Token::Function),
                | "break"    => Some(Token::Break),
                | "of"       => Some(Token::Of),
                | "end"      => Some(Token::End),
                | "in"       => Some(Token::In),
                | "nil"      => Some(Token::Nil),
                | "let"      => Some(Token::Let),
                | "do"       => Some(Token::Do),
                | "to"       => Some(Token::To),
                | "for"      => Some(Token::For),
                | "while"    => Some(Token::While),
                | "else"     => Some(Token::Else),
                | "then"     => Some(Token::Then),
                | "if"       => Some(Token::If),
                | "array"    => Some(Token::Array),
                | _          => None,
                };

                // Successfully lexed keyword
                if let Some(token) = token { return success(start, end, token); }

                // Check for identifier
                match ident {
                | "" => (),
                | id => return success(start, end, Token::Ident(sym::store(id))),
                };

                // Check for literal int
                match self.take_int(start) {
                | (_, "")                              => (),
                | (end, n) if i32::from_str(n).is_ok() => return success(start, end, Token::Int(i32::from_str(n).unwrap())),
                | (end, _)                             => return error(start, end, LexError::Integer),
                };

                // Check for literal string
                match self.take_string(start) {
                | (_, "")  => (),
                | (end, _) => {
                    // Cut off literal quotation marks
                    let string = String::from(self.slice(start + ByteOffset(1), end - ByteOffset(1)));
                    return success(start, end, Token::Str(string));
                },
                };

                // Failure to lex: consume until next whitespace and throw error
                let (end, _) = self.take_until(start, is_whitespace);
                return error(start, end, LexError::Unknown);
            },

            | Mode::Comment => {

                self.skip();

                match c {
                | '/' => if self.test_peek(|c| c == '*') { self.skip(); comment_level += 1 },
                | '*' => if self.test_peek(|c| c == '/') {
                            self.skip();
                            if comment_level == 0 {
                                self.mode = Mode::Source;
                            } else {
                                comment_level -= 1
                            }
                         },
                | _   => (),
                };
            },
            };
        }

        None
    }
}
