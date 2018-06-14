use std::fmt;

/// Represents all valid lexical tokens in the Tiger language.
#[derive(Debug)]
pub enum Token {

    // Keywords

    /// `type`
    Type,

    /// `var`
    Var,

    /// `function`
    Function,

    /// `break`
    Break,

    /// `of`
    Of,

    /// `end`
    End,

    /// `in`
    In,

    /// `nil`
    Nil,

    /// `let`
    Let,

    /// `do`
    Do,

    /// `to`
    To,

    /// `for`
    For,

    /// `while`
    While,

    /// `else`
    Else,

    /// `then`
    Then,

    /// `if`
    If,

    /// `Array`
    Array,

    // Operators

    /// `:=`
    Assign,

    /// `|`
    LOr,

    /// `&`
    LAnd,

    /// `>=`
    Ge,

    /// `>`
    Gt,

    /// `<=`
    Le,

    /// `<`
    Lt,

    /// `<>`
    Neq,

    /// `=`
    Eq,

    /// `/`
    Div,

    /// `*`
    Mul,

    /// `-`
    Sub,

    /// `+`
    Add,

    /// `.`
    Dot,

    // Miscellaneous

    /// `[`
    LBrace,

    /// `]`
    RBrace,

    /// `{`
    LBrack,

    /// `}`
    RBrack,

    /// `(`
    LParen,

    /// `)`
    RParen,
    
    /// `;`
    Semicolon,

    /// `:`
    Colon,

    /// `,`
    Comma,

    // Literals
    
    Int(i32),

    Str(String),

    Ident(String),
}

/// Token pretty printer
impl fmt::Display for Token {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
        | Token::Type          => write!(fmt, "KEYWORD type"),
        | Token::Var           => write!(fmt, "KEYWORD var"),
        | Token::Function      => write!(fmt, "KEYWORD function"),
        | Token::Break         => write!(fmt, "KEYWORD break"),
        | Token::Of            => write!(fmt, "KEYWORD of"),
        | Token::End           => write!(fmt, "KEYWORD end"),
        | Token::In            => write!(fmt, "KEYWORD in"),
        | Token::Nil           => write!(fmt, "KEYWORD nil"),
        | Token::Let           => write!(fmt, "KEYWORD let"),
        | Token::Do            => write!(fmt, "KEYWORD do"),
        | Token::To            => write!(fmt, "KEYWORD to"),
        | Token::For           => write!(fmt, "KEYWORD for"),
        | Token::While         => write!(fmt, "KEYWORD while"),
        | Token::Else          => write!(fmt, "KEYWORD else"),
        | Token::Then          => write!(fmt, "KEYWORD then"),
        | Token::If            => write!(fmt, "KEYWORD if"),
        | Token::Array         => write!(fmt, "KEYWORD array"),
        | Token::Assign        => write!(fmt, "OPERATOR :="),
        | Token::LOr           => write!(fmt, "OPERATOR |"),
        | Token::LAnd          => write!(fmt, "OPERATOR &"),
        | Token::Ge            => write!(fmt, "OPERATOR >="),
        | Token::Gt            => write!(fmt, "OPERATOR >"),
        | Token::Le            => write!(fmt, "OPERATOR <="),
        | Token::Lt            => write!(fmt, "OPERATOR <"),
        | Token::Neq           => write!(fmt, "OPERATOR <>"),
        | Token::Eq            => write!(fmt, "OPERATOR ="),
        | Token::Div           => write!(fmt, "OPERATOR /"),
        | Token::Mul           => write!(fmt, "OPERATOR *"),
        | Token::Sub           => write!(fmt, "OPERATOR -"),
        | Token::Add           => write!(fmt, "OPERATOR +"),
        | Token::Dot           => write!(fmt, "OPERATOR ."),
        | Token::LBrace        => write!(fmt, "SYMBOL ["),
        | Token::RBrace        => write!(fmt, "SYMBOL ]"),
        | Token::LBrack        => write!(fmt, "SYMBOL {{"), // Escape format string
        | Token::RBrack        => write!(fmt, "SYMBOL }}"),
        | Token::LParen        => write!(fmt, "SYMBOL ("),
        | Token::RParen        => write!(fmt, "SYMBOL )"),
        | Token::Semicolon     => write!(fmt, "SYMBOL ;"),
        | Token::Colon         => write!(fmt, "SYMBOL :"),
        | Token::Comma         => write!(fmt, "SYMBOL ,"),
        | Token::Int(n)        => write!(fmt, "INTEGER {}", n),
        | Token::Str(s)        => write!(fmt, "STRING \"{}\"", s),
        | Token::Ident(i)      => write!(fmt, "IDENTIFIER {}", i),
        }
    }
}
