#[derive(Debug)]
pub struct LexError {
    span: (usize, usize),
    code: LexErrorCode,     
}

#[derive(Debug)]
pub enum LexErrorCode {
    UnknownSymbol,
    UnknownToken,
    UnopenedComment,
    InvalidInteger,
}

impl LexError {

    pub fn new(start: usize, end: usize, code: LexErrorCode) -> Self {
        LexError {
            span: (start, end),
            code,
        }
    }

}
