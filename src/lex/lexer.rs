use std::str::CharIndices;
use std::str::FromStr;

use token::Token;
use lex::{LexError, LexErrorCode};

pub struct Lexer<'input> {
    mode: Mode,
    source: &'input str,
    stream: CharIndices<'input>,
    next: Option<(usize, char)>,
}

enum Mode {
    Source,
    Comment,
}

type Spanned = Result<(usize, Token, usize), LexError>;

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

    pub fn new(source: &'input str) -> Self {
        let mut stream = source.char_indices();
        let next = stream.next();
        Lexer { mode: Mode::Source, source, stream, next }
    }

    fn skip(&mut self) {
        self.next = self.stream.next();
    }

    fn peek(&self) -> Option<(usize, char)> {
        self.next
    }

    fn slice(&self, start: usize, end: usize) -> &'input str {
        &self.source[start..end]
    }

    fn take_while<F>(&mut self, start: usize, mut condition: F) -> (usize, &'input str)
        where F: FnMut(char) -> bool
    {
        self.take_until(start, |c| !condition(c))
    }

    fn take_until<F>(&mut self, start: usize, mut stop: F) -> (usize, &'input str)
        where F: FnMut(char) -> bool
    {
        while let Some((end, c)) = self.peek() {
            match stop(c) {
            | true  => return (end, self.slice(start, end)),
            | false => self.skip(),
            }
        }

        let eof = self.source.len();
        (eof, self.slice(start, eof))
    }

    fn take_symbol(&mut self, start: usize) -> (usize, &'input str) {
        self.take_while(start, is_symbol)
    }

    fn take_ident(&mut self, start: usize) -> (usize, &'input str) {
        let valid = self.peek().map(|(_, c)| is_ident_start(c)).unwrap_or(false);
        if !valid { return (start, "") }
        let (end, _) = self.take_while(start + 1, is_ident);
        (end, self.slice(start, end))
    }

    fn take_int(&mut self, start: usize) -> (usize, &'input str) {
        self.take_while(start, is_digit)
    }

    fn take_string(&mut self, start: usize) -> (usize, &'input str) {
        let valid = self.peek().map(|(_, c)| c == '"').unwrap_or(false);
        if !valid { return (start, "") }
        let (end, _) = self.take_until(start, |c| c == '"');
        self.skip();
        (end, self.slice(start + 1, end - 1))
    }

}

impl <'input> Iterator for Lexer<'input> {

    type Item = Spanned;

    fn next(&mut self) -> Option<Self::Item> {

        let mut comment_level = 0;

        while let Some((start, c)) = self.peek() {
            
            if is_whitespace(c) { self.skip(); continue }

            match self.mode {
            | Mode::Source => {

                // Look for symbol
                let (end, symbol) = self.take_symbol(start);
                let token = match symbol {
                | ""   => None,
                | ":=" => Some(Token::Assign),
                | "|"  => Some(Token::Or),
                | "&"  => Some(Token::And),
                | ">=" => Some(Token::Ge),
                | ">"  => Some(Token::Gt),
                | "<=" => Some(Token::Le),
                | "<"  => Some(Token::Lt),
                | "<>" => Some(Token::Neq),
                | "="  => Some(Token::Eq),
                | "/"  => Some(Token::Div),
                | "*"  => Some(Token::Mul),
                | "-"  => Some(Token::Sub),
                | "+"  => Some(Token::Add),
                | "."  => Some(Token::Dot),
                | "["  => Some(Token::LBrace),
                | "]"  => Some(Token::RBrace),
                | "{"  => Some(Token::LBrack),
                | "}"  => Some(Token::RBrack),
                | "("  => Some(Token::LParen),
                | ")"  => Some(Token::RParen),
                | ";"  => Some(Token::Semicolon),
                | ":"  => Some(Token::Colon),
                | ","  => Some(Token::Comma),
                | "/*" => { self.mode = Mode::Comment; continue; },
                | "*/" => return Some(Err(LexError::new(start, end, LexErrorCode::UnopenedComment))),
                | _    => return Some(Err(LexError::new(start, end, LexErrorCode::UnknownSymbol))),
                };

                // Successfully lexed symbol
                if let Some(token) = token { return Some(Ok((start, token, end))); }

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
                if let Some(token) = token { return Some(Ok((start, token, end))); }

                // Check for identifier
                match ident {
                | "" => (),
                | id => return Some(Ok((start, Token::Ident(String::from(id)), end))),
                };

                // Check for literal int
                match self.take_int(start) {
                | (_, "")                              => (),
                | (end, n) if i32::from_str(n).is_ok() => return Some(Ok((start, Token::Int(i32::from_str(n).unwrap()), end))),
                | (end, _)                             => return Some(Err(LexError::new(start, end, LexErrorCode::InvalidInteger))),
                };

                // Check for literal string
                match self.take_string(start) {
                | (_, "")  => (),
                | (end, s) => return Some(Ok((start, Token::Str(String::from(s)), end))),
                };

                // Failure to lex: consume until next whitespace and throw error
                let (end, _) = self.take_until(start, is_whitespace);
                return Some(Err(LexError::new(start, end, LexErrorCode::UnknownToken)));
            },

            | Mode::Comment => {

                let (_, symbol) = self.take_symbol(start);
                match symbol {
                | "/*" => comment_level += 1,
                | "*/" => if comment_level == 0 { self.mode = Mode::Source; },
                | ""   => self.skip(),
                | _    => (),
                };
            },
            };
        }

        None
    }
}
