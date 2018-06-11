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
